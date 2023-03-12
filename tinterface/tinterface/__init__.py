from pathlib import Path
import os, shutil, time
from typing import Tuple
import pexpect

TMODLOADER_DIR = str(Path('~/.local/share/Steam/steamapps/common/tModLoader/').expanduser()) + '/'
COMPUTERRARIA_DIR = os.path.join(os.path.dirname(__file__), '../../')
TMP_DIR = '/tmp/'

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
    """A representation of the world config for WiringUtils"""
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
    """

    def __init__(
        self, 
        path=None,
        world=None,
        port=7777,
        inplace=False,
        verbose=False,
    ):
        self.path = TMODLOADER_DIR + 'LaunchUtils/ScriptCaller.sh' if path is None else path
        self.world = COMPUTERRARIA_DIR + 'computer.wld' if world is None else world
        self.port = port
        self.inplace = inplace
        self.verbose = verbose

        # World specific config
        # Most of the random literal numbers should be here
        self.config_x = LoadConfig(46, 2, 3, 1024, 3185, 2)
        self.config_y = LoadConfig(421, 1, 4, 32, 156, 12)
        self.triggers = {
            'dummy1': (3197, 144), # dummy clock, 1 dummy
            'dummy40-1': (3189, 49), # dummy clock, 40 dummies
            'dummy40-2': (3177, 49),
            'dummy40-3': (3129, 49),
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
        if self.verbose: print(command)
        self.process = pexpect.spawn(' '.join(command), timeout=45)

        if self.verbose: print('Started server, waiting for completion')
        self.process.expect('Server started')
        time.sleep(0.2)
        self.process.sendline('init')
        time.sleep(0.2)
        self.process.sendline(f'bin config {self.config_x.to_str()} {self.config_y.to_str()}')
        time.sleep(0.2)
        if self.verbose: print('Server started successfully')

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
            # Required specification for WiringUtils
            with open(txtfile, 'w') as f:
                # hexdump -ve '1/1 "%.2x "' | head -c -1 > 
                # Need to trim since WiringUtils doesn't like a trailing space
                f.write(pexpect.run(f'hexdump -ve \'1/1 "%.2x "\' {binfile}').decode('utf-8')[:-1])
        # Sync here to avoid accidentally writing without syncing first
        self.sync()
        # WiringUtils has weird case sensitive glitch that I don't feel like fixing
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
        # WiringUtils has weird case sensitive glitch that I don't feel like fixing
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
        self.process.expect(': (?P<val>[0|1])\r\n')

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

    ###########################################################################
    # Higher level functions specific to computerraria
    ###########################################################################

    def reset_state(self):
        """Resets all state of the computer to a blank slate except ram"""
        assert(self.running())

        tries = 0
        # Prevent trying to reset state while in the middle of execution
        while not self.read(self.tiles['inexec']):
            self.trigger(self.triggers['clk'])
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

    def write_zeros(self):
        """Writes all zeros to memory"""
        assert(self.running())
        # Touch empty file
        empty = TMP_DIR + 'empty.txt'
        with open(empty, 'w') as _: pass
        self.write_bin(empty)

    
    def run(self, prog_file: str, out_file: str, run_time=15):
        """Runs prog_file and returns output to out_file"""
        assert(self.running())
        self.reset_state()
        self.write_bin(prog_file)
        self.trigger(self.triggers['dummy40-1'])
        self.trigger(self.triggers['dummy40-2'])
        self.trigger(self.triggers['dummy40-3'])

        # TODO: add in game interface to communicate finish

        time.sleep(run_time)

        self.trigger(self.triggers['dummy40-1'])
        self.trigger(self.triggers['dummy40-2'])
        self.trigger(self.triggers['dummy40-3'])
        # Very important to sync here, trigger is not thread safe unless we wait until things are synced
        self.sync()
        self.reset_state()
        # read_bin handles syncing
        self.read_bin(out_file)



        









