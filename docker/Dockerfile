# syntax = docker/dockerfile:1.2

################################################################################
# CI dockerfile
################################################################################
# This dockerfile contains everything required to run a full CI test suite
#
# One could potentially also use this for development, as long as another
# computer with a Terraria client installed can connect


################################################################################
# Base
################################################################################
# Probably should have used this in hindsight:
# https://github.com/tModLoader/tModLoader/tree/1.4/patches/tModLoader/Terraria/release_extras/DedicatedServerUtils
FROM steamcmd/steamcmd@sha256:0d3d8716fb0ac76f02bd9d850334b892d1caf92632fcce362bb52592d63211c3


################################################################################
# Base
################################################################################
# Yeah I'm not supposed to install user applications (vim), but it helps when 
# debugging and is trivially lightweight
RUN apt-get update && apt-get install -y \
    bsdmainutils \
    curl \
    gcc \
    git \
    libsdl2-2.0-0 \
    python3 \
    python3-pip \
    python3-tqdm \
    make \
    vim \
    wget

# libicu70 \


################################################################################
# tModLoader
################################################################################
RUN --mount=type=secret,id=_env,dst=/etc/secrets/.env \
    echo exit | steamcmd \
        "+login $(sed -n 1p /etc/secrets/.env) $(sed -n 2p /etc/secrets/.env)" \
        "+app_update 1281930 validate" &&\
    mkdir -p /root/.local/share/Terraria/tModLoader/ModSources &&\
    mkdir -p /root/.local/share/Terraria/tModLoader/Mods
# ln -s /root/Steam/steamapps/ /root/.local/share/Steam/ &&\
COPY tModLoader.targets /root/.local/share/Terraria/tModLoader/ModSources


################################################################################
# RISCV Toolchain
################################################################################
# These are kind of sketchy prebuilt binaries, but building from scratch takes 
# forever
WORKDIR /opt
RUN mkdir /opt/riscv &&\
    wget https://github.com/stnolting/riscv-gcc-prebuilt/releases/download/rv32i-4.0.0/riscv32-unknown-elf.gcc-12.1.0.tar.gz &&\
    tar -xzf riscv32-unknown-elf.gcc-12.1.0.tar.gz -C /opt/riscv/ &&\
    rm riscv32-unknown-elf.gcc-12.1.0.tar.gz
WORKDIR /root


################################################################################
# Rust
################################################################################
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh &&\
    sh rustup.sh -y --profile minimal &&\
    source "$HOME/.cargo/env" &&\
    rustup target add riscv32i-unknown-none-elf &&\
    rustup component add llvm-tools-preview &&\
    cargo install cargo-binutils &&\
    rm -rf /root/.cargo/registry &&\
    rustup target remove x86_64-unknown-linux-gnu
ENV PATH "$PATH:/opt/riscv/bin"


################################################################################
# Dotnet
################################################################################
RUN wget https://dot.net/v1/dotnet-install.sh &&\
    bash ./dotnet-install.sh --install-dir /usr/local/bin -channel 8.0 &&\
    dotnet --list-sdks


################################################################################
# Riscof
################################################################################
# RUN pip3 install git+https://github.com/riscv/riscof.git
RUN pip3 install --break-system-packages git+https://github.com/riscv/riscof.git
# I should set this up to be built, right now it relies on host machine
# already having it installed
# This would require setting up ocaml as well as other build dependencies
COPY riscv_sim_RV32 /usr/local/bin


################################################################################
# Entry point
################################################################################
# Override default entrypoint/cmd from base image
ENTRYPOINT ["/usr/bin/env"]
CMD ["/bin/bash"]

