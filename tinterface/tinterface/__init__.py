from subprocess import Popen, run, PIPE
from pathlib import Path
import os, shutil, time

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
        path='~/.local/share/Steam/steamapps/common/tModLoader/start-tModLoaderServer.sh',
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

    def start(self):
        """Starts the server, blocks until fully open"""
        self.process = Popen([
            self.path, 
            '-nosteam',
            f'-port {self.port}',
            f'-players 1',
            f'-world {self.world}',
         ], stdin=PIPE, stdout=PIPE, stderr=PIPE)

        assert(self.process.stdin is not None)
        assert(self.process.stdout is not None)
        assert(self.process.stderr is not None)

        print('Started server, waiting for completion')
        # This block indefinitely if something fails, should probably have a timeout or something
        while self.process.stdout.readline() != b'Server started\n': pass
        self.process.stdin.write(b'init\n')
        print('Server started successfully')

    def stop(self):
        """Stops the server and cleans up"""
        # Make double sure you're not deleting the actual world
        if not self.inplace and Path(self.world).parts[1] == 'tmp':
            os.remove(self.world)
        if self.process is not None:
            self.process.stdin.write(b'exit\n') 
            while self.running(): time.sleep(0.1)

    def running(self) -> bool:
        return self.process is not None and self.process.poll() is None
            

    def config(self, config_x, config_y):
        """Sets config of the world for mass reads/writes"""
        assert(self.running())
        self.process.stdin.write(f'bin config {self.config_x.to_str()} {self.config_y.to_str()}\n')

    def write(self, file: str):
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
        self.process.stdin.write(f'bin write {txtfile}')








