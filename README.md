<h1 align="center">
  <picture><img src="./doc/img/logo.png" height="400"/></picture>
  <br />
  Computerraria
</h1>
<h2 align="center">
  A fully compliant RISC-V computer inside Terraria
</h2>
<div align="center">
  <a href=https://github.com/misprit7/computerraria/actions/workflows/tests.yml>
    <img src=https://github.com/misprit7/computerraria/actions/workflows/tests.yml/badge.svg/>
  </a>
</div>

# Pitch

There are two fundamentally competing forces when it comes to computer speed. The first, and most famous, is Moore's law, where physical transistor densities scale exponentially. The second is the inevitable growth of software bloat that runs on top of increasingly modern processors. There's a kind of equilibrium between these two competing beasts, ensuring that a user always manages to get at least a split second of mindfulness while staring at a frozen screen whenever attempting to open the latest app. 

This project is an attempt to score a decisive route in this ongoing battle in favor of the *programmer*. By emulating a complete rv32i instruction set inside the wiring system of [Terraria](https://www.terraria.org/), we push back speeds to the early 70s era, tossing the ball firmly back into the court of silicon engineer without losing any software functionality. 

# Specs

Despite what the pitch may lead one to believe the goal of this project is to maximize the compliance and processing ability of the in game cpu. This is only possible with the help of an accelerator mod, which maintains full compatibility with the vanilla wiring system but reimplements it in a much more efficient manner:

[WireHead](https://github.com/misprit7/WireHead) - A wiring accelerator and headless control mod

With this installed, the current specs of the computer are as follows: 

- Clock speed: ~5kHz
- Ram: 96kb
- Instruction set: rv32i

# File Structure

The major relevant parts of the project are as follows:

```
.
├── app/
│   └── tdriver/
├── computer.wld
├── copy_world.sh
├── doc/
├── docker/
├── test/
└── tinterface/
    ├── bin/
    └── tinterface/
```

## app/

Higher level applications to be run on the computer, not including compliance tests. Currently all in rust but could also easily be in C. 

### tdriver/

Driver API for interacting cpu from rust. 

## computer.wld

The actual world file. This is technically a binary file, but given the context of the project it acts much more like source code given that it is edited manually and compresses extremely well. This generally isn't edited in place, it's copied back and forth to the user installation with [copy-world.sh](copy_world.sh). 

## doc/

Documentation/notes for the project

## docker/

Files required to build docker image for CI. 

## test/

All automated tests written for the CPU. These are mostly handled through [riscof](https://github.com/riscv-software-src/riscof). This consists of both the computerraria plugin as well as a reference plugin ([sail_cSim](test/sail_cSim/)) to compare the results to. 

## tinterface/

Interfaces programmatically with running Terraria instance. This consists of both a python module as well as a command line wrapper to upload binaries, start execution and manipulate other fine grain controls without needing a GUI. 

# Setup

## Docker

The easiest way to get setup is with the docker image available here: 

[Docker Image](https://hub.docker.com/r/misprit7/computerraria)

If you already have docker installed this can be pulled with

```bash
docker pull misprit7/computerraria
```

You can then start the container with

```bash
docker run -it misprit7/computerraria
```

This image already has all tooling installed so you should be able to build everything. 

