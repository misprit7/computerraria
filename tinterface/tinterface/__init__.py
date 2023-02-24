from pathlib import Path
import os, shutil, time
from typing import Tuple
import pexpect

# pyright complains about stdin/stdout otherwise
# pyright: reportOptionalMemberAccess=false

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
        return self.offset + self.cell_gap * self.cells * self.banks + self.bank_gap * (self.banks - 1)

class TServer:
    """A class representing a terraria server instance"""

    def __init__(
        self, 
        path='~/.local/share/Steam/steamapps/common/tModLoader/LaunchUtils/ScriptCaller.sh',
        world=os.path.join(os.path.dirname(__file__), '../../computer.wld'),
        port=7777,
        inplace=False,
    ):
        self.path = str(Path(path).expanduser())
        self.port = port
        self.world = world
        self.process: pexpect.spawn|None = None
        self.inplace = inplace

        # World specific config
        self.config_x = LoadConfig(814, 2, 3, 1024, 3361, 2)
        self.config_y = LoadConfig(1377, 1, 4, 32, 0, 1)
        self.triggers = {
            'dummy': (3965, 1100),
            'clk': (3969, 1114),
            'reset': (3966, 1112),
            'zdb': (4011, 1182),
            'zmem': (3962, 1330),
            'lpc': (3966, 1154),
        }

        if not self.inplace:
            self.world = shutil.copy(self.world, '/tmp/')

    def start(self, verbose=False):
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
        if verbose: print(command)
        self.process = pexpect.spawn(' '.join(command))

        if verbose: print('Started server, waiting for completion')
        self.process.expect('Server started')
        time.sleep(0.2)
        self.process.sendline('init')
        time.sleep(0.2)
        if verbose: print('Server started successfully')

    def stop(self):
        """Stops the server and cleans up"""
        # Make double sure you're not deleting the actual world
        if not self.inplace and Path(self.world).parts[1] == 'tmp':
            os.remove(self.world)
        if self.process is not None:
            self.process.sendline('exit')
            while self.running(): time.sleep(0.1)

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
            pexpect.run(f'riscv32-unknown-elf-objdump -O binary {file} {binfile}')
        if ext == '.bin' or ext == '.elf':
            # Required specification for WiringUtils
            with open(txtfile, 'w') as f:
                f.write(pexpect.run(f'hexdump -ve \'1/1 "%.2x "\' {binfile}').decode('utf-8'))
        self.process.sendline(f'bin write {txtfile}')

    def read_bin(self, file: str, force=False):
        """Reads world bin into a file"""
        assert(self.running())
        _, ext = os.path.splitext(file)
        if not force:
            # Unless forced make sure you don't overwrite a non txt file
            assert(ext == '.txt')
        self.process.sendline(f'bin read {file}')

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
        self.process.expect(': (?P<val>[0|1])')
        b = int(self.process.match.group('val'))
        if b == 1: return True
        elif b == 0: return False
        else: raise ValueError('Read returned neither 0 nor 1')

    def trigger(self, coord: Tuple[int, int]):
        """Triggers a specific tile in the world"""
        assert(self.running())
        x, y = coord
        self.process.sendline(f'trigger {x} {y}')

    ###########################################################################
    # Higher level functions specific to computerraria
    ###########################################################################

    def reset_state(self):
        """Resets all state of the computer to a blank slate except ram"""
        assert(self.running())
        self.trigger(self.triggers['zdb'])
        self.trigger(self.triggers['zmem'])

    def write_zeros(self):
        """Writes all zeros to memory"""
        assert(self.running())
        # Touch empty file
        empty = '/tmp/empty.txt'
        with open(empty, 'w') as _: pass
        self.write_bin(empty)

    
    def run(self, prog_file: str, out_file: str):
        """Runs prog_file and returns output to out_file"""
        assert(self.running())
        self.reset_state()
        self.write_bin(prog_file)
        self.trigger(self.triggers['dummy'])

        monitor = (self.config_x.end(), self.config_y.end())
        t_start = time.time()
        while not self.read(monitor):
            time.sleep(0.2)
            if time.time() - t_start > 20:
                self.trigger(self.triggers['dummy'])
                raise TimeoutError('Program timed out while executing')

        self.trigger(self.triggers['dummy'])
        self.reset_state()
        self.read_bin(out_file)



        









