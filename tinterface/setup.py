from setuptools import setup

setup(
    name='tinterface',
    version='0.1',
    description='Terraria server python/cli interface',
    url='https://github.com/misprit7/computerraria',
    author='Xander Naumenko',
    author_email='xandernaumenko@gmail.com',
    license='MIT',
    packages=['tinterface'],
    zip_safe=False,
    scripts=['bin/tinterface-cli'],
)
