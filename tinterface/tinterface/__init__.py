from subprocess import Popen, run, PIPE
from pathlib import Path
import os, shutil, time, select, sys

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
        self.process: Popen | None = None
        self.config_x = LoadConfig(814, 2, 3, 1024, 3361, 2)
        self.config_y = LoadConfig(1377, 1, 4, 32, 0, 1)
        self.inplace = inplace

        if not self.inplace:
            self.world = shutil.copy(self.world, '/tmp/')

    def clear_stdout(self):
        """Clears self.process.stdout"""
        assert(self.running())
        # This is stupidly complicated for something so simple: 
        # https://repolinux.wordpress.com/2012/10/09/non-blocking-read-from-stdin-in-python/
        while self.process.stdout in select.select([self.process.stdout], [], [], 0)[0]:
            self.process.stdout.readline()

    def write_stdin(self, s: str):
        self.process.stdin.write(str.encode(s + '\n'))
        self.process.stdin.flush()

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
        self.process = Popen(command, stdin=PIPE, stdout=PIPE, stderr=PIPE, bufsize=1, text=True)

        assert(self.process.stdin is not None)
        assert(self.process.stdout is not None)
        assert(self.process.stderr is not None)

        print('Started server, waiting for completion')
        # This block indefinitely if something fails, should probably have a timeout or something
        line = self.process.stdout.readline()
        while line != b'Server started\n':
            if verbose and line != b'':
                print(line)
            line = self.process.stdout.readline()


        self.write_stdin('init')
        print('Server started successfully')

    def stop(self):
        """Stops the server and cleans up"""
        # Make double sure you're not deleting the actual world
        if not self.inplace and Path(self.world).parts[1] == 'tmp':
            os.remove(self.world)
        if self.process is not None:
            self.write_stdin('exit') 
            while self.running(): time.sleep(0.1)

    def running(self) -> bool:
        return self.process is not None and self.process.poll() is None
            

    def config(self, config_x, config_y):
        """Sets config of the world for mass reads/writes"""
        assert(self.running())
        self.write_stdin(f'bin config {self.config_x.to_str()} {self.config_y.to_str()}')

    def write_bin(self, file: str):
        """Writes given file to the world, if given an elf it will convert to bin in place"""
        assert(self.running())
        f, ext = os.path.splitext(file)
        assert(ext == '.bin' or ext == '.elf' or ext == '.bin')
        binfile = f + '.bin'
        txtfile = f + '.txt'
        if ext == '.elf':
            run(['riscv32-unknown-elf-objdump', '-O', 'binary', file, binfile], check = True)
        if ext == '.bin' or ext == '.elf':
            # Required specification for WiringUtils
            run(['hexdump', '-ve', '1/1 "%.2x "', binfile, '>', txtfile], check = True)
        self.write_stdin(f'bin write {txtfile}')

    def read_bin(self, file: str, force=False):
        """Reads world bin into a file"""
        assert(self.running())
        _, ext = os.path.splitext(file)
        if not force:
            # Unless forced make sure you don't overwrite a non txt file
            assert(ext == '.txt')
        self.write_stdin(f'bin read {file}')

    def write(self, x: int, y: int, val: bool):
        assert(self.running())
        self.write_stdin(f'write {x} {y} {1 if val else 0}')

    def read(self, x: int, y: int) -> bool:
        assert(self.running())
        self.clear_stdout()
        self.write_stdin(f'read {x} {y}')
        l = self.process.stdout.readline()
        # Format: ': 1\n' or ': 0\n'
        c = l[2]
        if c == b'1': return True
        elif c == b'0': return False
        else: raise ValueError('Read returned neither 0 nor 1')

        









