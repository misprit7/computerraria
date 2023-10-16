from pathlib import Path
import os, shutil, time, sys
from typing import Tuple
import pexpect
from datetime import datetime
from tqdm import tqdm

# CI image is under root user
TMODLOADER_DIR = str(Path('/root/.local/share/Steam/steamapps/common/tModLoader/')) + '/'
if not os.path.isdir(TMODLOADER_DIR):
    TMODLOADER_DIR = str(Path('~/.local/share/Steam/steamapps/common/tModLoader/').expanduser()) + '/'
# This only works if installed with pip install -e, otherwise it is up to the user to find world
COMPUTERRARIA_DIR = os.path.join(os.path.dirname(__file__), '../../')
TMP_DIR = '/tmp/'

FPS = 60

###########################################################################
# Functions
###########################################################################

def gen_signature(txt_file: str, sig_file: str, offset: int):
    """
    Generates a signature file from an output file

    txt_file: txt file from /bin read in game
    sig_file: signature file to write to, compliant with riscof specs
    offset: offset in ram to read from
    """
    with open(txt_file, 'r') as txt, open(sig_file, 'w') as sig:
        bytes = txt.read().lower().split()[offset:]
        # This technically isn't robust if the start/end signature ends with 00
        while bytes[-1] == '00': bytes.pop()
        # For some undocumented reason the number of lines to be printed must be divisible by 4
        while len(bytes)%16 != 0: bytes.append('00')
        for i in range(0, len(bytes), 4):
            # Reverse since roman numbers are big endian while we're little endian
            sig.write(''.join(bytes[i:i+4][::-1]))
            sig.write('\n')

###########################################################################
# Classes
###########################################################################

class LoadConfig:
    """A representation of the world config for WireHead"""
    def __init__(self, offset: int, cell_width: int, cell_gap: int, cells: int, bank_gap: int, banks: int):
        self.offset=offset
        self.cell_width=cell_width
        self.cell_gap=cell_gap
        self.cells=cells
        self.bank_gap=bank_gap
        self.banks=banks

    def to_str(self):
        return f'{self.offset}+{self.cell_width}g{self.cell_gap}x{self.cells}g{self.bank_gap}x{self.banks}'

    def end(self) -> int:
        return self.offset + self.cell_gap * (self.cells-1) + self.bank_gap * (self.banks - 1)

class TServer:
    """
    A class representing a terraria server instance

    Methods should block until execution of command is finished

    Useful tips: 
    - Use tserver.process.interact() to debug server, ctrl+] to exit back to python
    - Never match an expect statement with anything compatible with the following two lines:
    Saving world data: 100%
    Validating world save: 100%
     ( or any percentage). They are sent randomly every few minutes
    """

    def __init__(
        self, 
        path=None,
        world=None,
        port=7777,
        inplace=False,
        verbose=False,
        terracc=False,
        lazy=False,
    ):
        self.path = TMODLOADER_DIR + 'LaunchUtils/ScriptCaller.sh' if path is None else path
        self.world = COMPUTERRARIA_DIR + 'computer.wld' if world is None else world
        self.port = port
        self.inplace = inplace
        self.verbose = verbose
        self.terracc = terracc
        self.lazy = lazy


        # World specific config
        # Most of the random literal numbers should be here
        self.config_x = LoadConfig(46, 2, 3, 1024, 3185, 2)
        self.config_y = LoadConfig(421, 1, 4, 32, 156, 12)
        self.triggers = {
            'dummy1': (3197, 144), # dummy clock, 1 dummy
            'dummy40-1': (3189, 49), # dummy clock, 40 dummies
            'dummy40-2': (3177, 49),
            'dummy40-3': (3129, 49),
            'dummy-tl': (3089, 53), # Individual dummies, top left
            'clk': (3201, 158), # manual clock
            'reset': (3198, 156), # reset
            'zdb': (3243, 226), # zero data bus
            'zmem': (3250, 374), # zero memory select
            'lpc': (3198, 198), # store pc
            'rst': (3198, 156), # resets clock
        }
        self.tiles = {
            'inexec': (3199, 156), # while in execution
        }
        self.dummies_gap = (12, 9)
        self.num_dummies = (12, 10)

        # Keep track of which dummies are on
        # indexed by [x][y]
        self.dummies = [[False for _ in range(self.num_dummies[1])] for _ in range(self.num_dummies[0])]

        if not self.inplace:
            self.world = shutil.copy(self.world, TMP_DIR)

    def start(self):
        """Starts the server, blocks until fully open"""
        command = [
            self.path, 
            '-server',
            '-port',
            str(self.port),
            '-players',
            '1',
            '-world',
            self.world,
        ]
        if self.verbose:print(command)
        self.process = pexpect.spawn(' '.join(command), timeout=45)

        if self.verbose:
            self.process.logfile = sys.stdout.buffer
        else:
            self.process.logfile = open(TMP_DIR + 'tinterface.log', 'wb')

        self.process.expect('Server started')
        print('Server started')
        time.sleep(0.2)
        self.process.sendline('init')
        time.sleep(0.2)
        self.process.sendline(f'bin config {self.config_x.to_str()} {self.config_y.to_str()}')
        time.sleep(0.2)
        self.clock_start(self.triggers['clk'], -1)
        time.sleep(0.2)
        if self.terracc:
            self.compile(lazy=self.lazy)
            print('terracc compiled')

    def stop(self):
        """Stops the server and cleans up"""
        # Make double sure you're not deleting the actual world
        if not self.inplace and Path(self.world).parts[1] == 'tmp':
            os.remove(self.world)
        if self.process is not None:
            self.process.sendline('exit')
            self.process.wait()

    def running(self) -> bool:
        return self.process is not None and self.process.isalive()
            
    ###########################################################################
    # Low level wrappers around mod commands
    ###########################################################################

    def config(self, config_x, config_y):
        """Sets config of the world for mass reads/writes"""
        assert(self.running())
        self.process.sendline(f'bin config {self.config_x.to_str()} {self.config_y.to_str()}')

    def compile(self, lazy=False):
        self.process.sendline('accel compile lazy' if lazy else 'accel compile')
        self.process.expect('terracc enabled', timeout=120)

    def write_bin(self, file: str):
        """Writes given file to the world, if given an elf it will convert to bin in place"""
        assert(self.running())
        f, ext = os.path.splitext(file)
        assert(ext == '.bin' or ext == '.elf' or ext == '.txt')
        binfile = f + '.bin'
        txtfile = f + '.txt'

        if ext == '.elf':
            objcopy = ''
            if shutil.which('rust-objcopy'):
                objcopy = 'rust-objcopy'
            elif shutil.which('riscv32-unknown-elf-objcopy'):
                objcopy = 'riscv32-unknown-elf-objcopy'
            else:
                print('No objdump utility found, write failed')
                return
            pexpect.run(f'{objcopy} -O binary {file} {binfile}')
        if ext == '.bin' or ext == '.elf':
            # Required specification for WireHead
            with open(txtfile, 'w') as f:
                # hexdump -ve '1/1 "%.2x "' | head -c -1 > 
                # Need to trim since WireHead doesn't like a trailing space
                f.write(pexpect.run(f'hexdump -ve \'1/1 "%.2x "\' {binfile}').decode('utf-8')[:-1])

        # Sync here to avoid accidentally writing without syncing first
        self.sync()
        # WireHead has weird case sensitive glitch that I don't feel like fixing
        write = TMP_DIR + 'write-bin.txt'
        shutil.copyfile(txtfile, write)
        self.process.sendline(f'bin write {write}')
        self.process.expect('Write complete')

    def read_bin(self, file: str, force=False):
        """Reads world bin into a file"""
        assert(self.running())
        _, ext = os.path.splitext(file)
        if not force:
            # Unless forced make sure you don't overwrite a non txt file
            assert(ext == '.txt')

        # Sync here to avoid accidentally reading without syncing first
        self.sync()
        # WireHead has weird case sensitive glitch that I don't feel like fixing
        read = TMP_DIR + 'read-bin.txt'
        self.process.sendline(f'bin read {read}')
        self.process.expect('Read complete')
        shutil.copyfile(read, file)

    def write(self, coord: Tuple[int, int], val: bool):
        """Writes a specific tile to the world"""
        assert(self.running())
        x, y = coord
        self.process.sendline(f'write {x} {y} {1 if val else 0}')

    def read(self, coord: Tuple[int, int]) -> bool:
        """Reads a specific tile from the world"""
        assert(self.running())

        x, y = coord
        self.process.sendline(f'read {x} {y}')
        self.process.expect('Read complete: (?P<val>[0|1])\r\n')

        if self.process.match is None: 
            raise ValueError('Read returned neither 0 nor 1')
        b = int(self.process.match.group('val'))
        if b == 1: return True
        elif b == 0: return False
        else: raise ValueError('Read returned neither 0 nor 1')

    def trigger(self, coord: Tuple[int, int]):
        """Triggers a specific tile in the world"""
        assert(self.running())
        x, y = coord
        self.process.sendline(f'trigger {x} {y}')
        self.process.expect('Trigger complete')

    def sync(self):
        """Sync accelerator"""
        assert(self.running())
        self.process.sendline(f'accel sync')
        self.process.expect('Sync complete')

    def preprocess(self):
        """Preprocess world file (should be done on world load automatically)"""
        assert(self.running())
        self.process.sendline(f'accel preprocess')
        self.process.expect('Preprocess complete')

    def accel_enabled(self, enabled: bool):
        """Set accelerator on or off"""
        assert(self.running())
        if enabled:
            self.process.sendline(f'accel enable')
            self.process.expect('Accelerator enabled')
        else:
            self.process.sendline(f'accel disable')
            self.process.expect('Accelerator disabled')

    def clock_start(self, coord: Tuple[int, int], count: int):
        """
        Registers a monitor on the clock
        """
        assert(self.running())
        x, y = coord
        self.process.sendline(f'monitor clock {x} {y} {count}')
        self.process.expect('Clock count started')

    def clock_wait(self):
        """Waits for the clock to finish"""
        assert(self.running())
        self.process.expect('Clock finished')

    def clock_count(self) -> int:
        """Gets the current clock count"""
        assert(self.running())
        self.process.sendline('monitor clock count')
        self.process.expect('Clock count complete: (?P<count>[0-9]+)\r\n')
        if self.process.match is None: 
            raise ValueError('Clock count didn\'t return an int')
        return int(self.process.match.group('count'))



    ###########################################################################
    # Higher level functions specific to computerraria
    ###########################################################################

    def reset_state(self):
        """Resets all state of the computer to a blank slate except ram"""
        assert(self.running())

        self.sync()
        tries = 0
        # Prevent trying to reset state while in the middle of execution
        while not self.read(self.tiles['inexec']):
            self.trigger(self.triggers['clk'])
            self.sync()
            tries += 1
            # Most clock cycles of an instructions is 3
            if tries >= 3: 
                if self.verbose:
                    print('Unable to break out of execution with clk, resetting...')
                self.trigger(self.triggers['rst'])
                break
            # Make sure io isn't faster than fps
            time.sleep(0.2)
        
        self.trigger(self.triggers['zdb'])
        self.trigger(self.triggers['zmem'])
        self.trigger(self.triggers['lpc'])
        self.sync()

    def write_zeros(self):
        """Writes all zeros to memory"""
        assert(self.running())
        # Touch empty file
        empty = TMP_DIR + 'empty.txt'
        with open(empty, 'w') as _: pass
        self.write_bin(empty)

    def set_freq(self, f: int):
        """
        Triggers a certain number of dummies to be active to match frequency
        """
        n = f // FPS
        X = range(self.num_dummies[0])
        Y = range(self.num_dummies[1])
        n_cur = sum((sum(a) for a in self.dummies))
        for x in X:
            for y in Y:
                if n_cur == n:
                    return
                if n_cur < n and not self.dummies[x][y]:
                    self.dummies[x][y] = True
                    self.trigger((self.triggers['dummy-tl'][0] + x * self.dummies_gap[0],
                                 self.triggers['dummy-tl'][1] + y * self.dummies_gap[1]))
                    n_cur += 1
                if n_cur > n and self.dummies[x][y]:
                    self.dummies[x][y] = False
                    self.trigger((self.triggers['dummy-tl'][0] + x * self.dummies_gap[0],
                                 self.triggers['dummy-tl'][1] + y * self.dummies_gap[1]))
                    n_cur -= 1

    def run(self, prog_file: str, out_file: str, clock_cycles=50000, timeout_s=300):
        """
        Runs prog_file and returns output to out_file
        Assumes world is in ready state
        """
        assert(self.running())
        self.write_bin(prog_file)

        time.sleep(0.5)

        if self.terracc:
            print('Started compiling')
            self.compile(lazy=False)
            print('Finished compiling')

        first_cc = self.clock_count()

        # Number of dummies active
        n_cur = 120
        self.set_freq(n_cur * FPS)

        self.sync()

        t_start = time.time()
        while self.clock_count() - first_cc < clock_cycles:
            time.sleep(1)
            if self.clock_count() == first_cc:
                print('Clock not progressing! Cancelling run')
                break
            if time.time() - t_start > timeout_s:
                print('Timed out! Cancelling run')
                break

        print('Finished execution')

        self.set_freq(0)

        self.reset_state()
        time.sleep(0.2)
        # read_bin handles syncing
        self.read_bin(out_file)

    def stress_test(self):
        print('Starting stress test')
        for _ in tqdm(range(5000)):
            self.trigger(self.triggers['clk'])
        print('Finished stress test')


