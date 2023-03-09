import os
import logging
import tinterface


import riscof.utils as utils
import riscof.constants as constants
from riscof.pluginTemplate import pluginTemplate

logger = logging.getLogger()

class computerraria(pluginTemplate):
  __model__ = "computerraria"

  __version__ = "2.0"

  def __init__(self, *args, **kwargs):
    super().__init__(*args, **kwargs)

    config = kwargs.get('config')

    if config is None:
      print("Please enter input file paths in configuration.")
      raise SystemExit(1)
    logger.info(config)

    # self.dut_exe = os.path.join(config['PATH'] if 'PATH' in config else \
    #                             "~/.local/share/Steam/steamapps/common/tModLoader","start-tModLoaderServer.sh")
    self.dut_exe = os.path.join(config['PATH'] if 'PATH' in config else \
                                "echo","start-tModLoaderServer.sh")


    self.num_jobs = str(config['jobs'] if 'jobs' in config else 1)
    self.pluginpath=os.path.abspath(config['pluginpath'])
    self.isa_spec = os.path.abspath(config['ispec'])
    self.platform_spec = os.path.abspath(config['pspec'])
    if 'target_run' in config and config['target_run']=='0':
      self.target_run = False
    else:
      self.target_run = True

  def initialise(self, suite, work_dir, archtest_env):

    self.work_dir = work_dir
    self.suite_dir = suite
    self.compile_cmd = 'riscv{1}-unknown-elf-gcc -march={0} \
      -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles -g\
      -T '+self.pluginpath+'/env/link.ld\
      -I '+self.pluginpath+'/env/\
      -I ' + archtest_env + ' {2} -o {3} {4}'

  def build(self, isa_yaml, platform_yaml):

    # load the isa yaml as a dictionary in python.
    ispec = utils.load_yaml(isa_yaml)['hart0']

    self.xlen = '32'
    self.isa = 'rv32I'
    self.compile_cmd = self.compile_cmd+' -mabi=ilp32'

  def runTests(self, testList):

    logger.info('Starting server')
    tserver = tinterface.TServer()
    tserver.start()
    logger.info('Server loaded')
    for testname in testList:
      logger.info('Running Test: {0} on DUT'.format(testname))

      # for each testname we get all its fields (as described by the testList format)
      testentry = testList[testname]
      test = testentry['test_path']
      test_dir = testentry['work_dir']

      elf = 'dut.elf'
      out = 'dut-out.txt'

      # name of the signature file as per requirement of RISCOF. RISCOF expects the signature to
      # be named as DUT-<dut-name>.signature. The below variable creates an absolute path of
      # signature file.
      sig_file = os.path.join(test_dir, self.name[:-1] + ".signature")
      elf_file = os.path.join(test_dir, elf)
      out_file = os.path.join(test_dir, out)
      compile_macros= ' -D' + " -D".join(testentry['macros'])

      cmd = self.compile_cmd.format(testentry['isa'].lower(), self.xlen, test, elf_file, compile_macros)

      logger.info('Compile command: ' + cmd)
      utils.shellCommand(cmd).run(cwd=test_dir)

      if not self.target_run:
        continue

      logger.info('Starting test')

      try:
        tserver.run(elf_file, out_file, run_time=10)
        # Don't love hardcoding this but riscof has made it difficult enough as it is for me
        # Seriously why don't they pass this as a parameter to runTests
        tinterface.gen_signature(out_file, sig_file, 0x10110)
      except Exception as e:
        logger.error(e)
        logger.error('An error occured, dropping into interactive mode to debug')
        tserver.process.interact()
        raise SystemExit

      logger.info('Test complete')



