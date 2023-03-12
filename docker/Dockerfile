# syntax = docker/dockerfile:1.2
# ^ needed for secrets
FROM steamcmd/steamcmd:latest

# Install tModLoader
RUN --mount=type=secret,id=_env,dst=/etc/secrets/.env \
    echo exit | steamcmd \
        "+login $(sed -n 1p /etc/secrets/.env) $(sed -n 2p /etc/secrets/.env)" \
        "+app_update 1281930 validate" &&\
    ln -s /root/Steam/steamapps/ /root/.local/share/Steam/

# riscv gnu toolchain
# These are kind of sketchy prebuilt binaries, but building from scratch takes forever
WORKDIR /opt
RUN mkdir /opt/riscv &&\
    wget https://github.com/stnolting/riscv-gcc-prebuilt/releases/download/rv32i-4.0.0/riscv32-unknown-elf.gcc-12.1.0.tar.gz &&\
    tar -xzf riscv32-unknown-elf.gcc-12.1.0.tar.gz -C /opt/riscv/ &&\
    rm riscv32-unknown-elf.gcc-12.1.0.tar.gz

# Put apt updates after large terraria/toolchain install so adding a dependency doesn't require rebuild
# Yeah I'm not supposed to install vim, but it helps when debugging and is trivially lightweight
# Pip is for some reason massive (300mb) unless you don't install recommended
RUN apt-get update && apt-get install -y \
    curl \
    gcc \
    git \
    libicu70 \
    python3 \
    vim \
    wget && \
    apt-get install -y --no-install-recommends \
    python3-pip

WORKDIR /root

# Rust stuff
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh &&\
    sh rustup.sh -y --profile minimal &&\
    source "$HOME/.cargo/env" &&\
    rustup target add riscv32i-unknown-none-elf &&\
    rustup component add llvm-tools-preview &&\
    cargo install cargo-binutils &&\
    rm -rf /root/.cargo/registry &&\
    rustup target remove x86_64-unknown-linux-gnu

ENV PATH "$PATH:/opt/riscv/bin"

WORKDIR /root

# Riscof
RUN pip3 install git+https://github.com/riscv/riscof.git


# Override default entrypoint/cmd from base image
ENTRYPOINT ["/usr/bin/env"]
CMD ["/bin/bash"]

