import os
import re
import shutil
import subprocess
import shlex
import logging
import random
import string
from string import Template
import sys

import riscof.utils as utils
import riscof.constants as constants
from riscof.pluginTemplate import pluginTemplate

logger = logging.getLogger()

class computerraria(pluginTemplate):
  __model__ = "computerraria"

  #TODO: please update the below to indicate family, version, etc of your DUT.
  __version__ = "XXX"

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

    # Delete Makefile if it already exists.
    if os.path.exists(self.work_dir+ "/Makefile." + self.name[:-1]):
      os.remove(self.work_dir+ "/Makefile." + self.name[:-1])
    # create an instance the makeUtil class that we will use to create targets.
    make = utils.makeUtil(makefilePath=os.path.join(self.work_dir, "Makefile." + self.name[:-1]))

    make.makeCommand = 'make -k -j' + self.num_jobs

    for testname in testList:

      # for each testname we get all its fields (as described by the testList format)
      testentry = testList[testname]
      test = testentry['test_path']
      # capture the directory where the artifacts of this test will be dumped/created. RISCOF is
      # going to look into this directory for the signature files
      test_dir = testentry['work_dir']

      elf = 'my.elf'

      # name of the signature file as per requirement of RISCOF. RISCOF expects the signature to
      # be named as DUT-<dut-name>.signature. The below variable creates an absolute path of
      # signature file.
      sig_file = os.path.join(test_dir, self.name[:-1] + ".signature")
      compile_macros= ' -D' + " -D".join(testentry['macros'])

      cmd = self.compile_cmd.format(testentry['isa'].lower(), self.xlen, test, elf, compile_macros)
      print(f"cmd: {cmd}")

      # if self.target_run:
      if False:
        # TODO: Change this to match terraria format
        simcmd = self.dut_exe + ' --isa={0} +signature={1} +signature-granularity=4 {2}'.format(self.isa, sig_file, elf)
      else:
        simcmd = 'echo "NO RUN"'

      execute = '@cd {0}; {1}; {2};'.format(testentry['work_dir'], cmd, simcmd)

      make.add_target(execute)

    # if you would like to exit the framework once the makefile generation is complete uncomment the
    # following line. Note this will prevent any signature checking or report generation.
    #raise SystemExit

    make.execute_all(self.work_dir)

    if not self.target_run:
      raise SystemExit(0)

