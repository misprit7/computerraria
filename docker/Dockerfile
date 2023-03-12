# syntax = docker/dockerfile:1.2
# ^ needed for secrets
FROM steamcmd/steamcmd:latest

# RUN --mount=type=secret,id=_env,dst=/etc/secrets/.env \
#     mkdir /opt/steamcmd &&\
#     cd /opt/steamcmd &&\
#     wget http://media.steampowered.com/installer/steamcmd_linux.tar.gz &&\
#     tar xvf steamcmd_linux.tar.gz &&\
#     rm steamcmd_linux.tar.gz\
#     ./steamcmd.sh "+login $(sed -n 1p /etc/secrets/.env) $(sed -n 2p /etc/secrets/.env)"&&\
#     ./steamcmd.sh app_update 1281930 validate

RUN --mount=type=secret,id=_env,dst=/etc/secrets/.env \
    echo exit | steamcmd \
        "+login $(sed -n 1p /etc/secrets/.env) $(sed -n 2p /etc/secrets/.env)" \
        "+app_update 1281930 validate" # tModLoader

# Put apt updates after large terraria install so adding a dependency doesn't require rebuild
# Yeah I'm not supposed to install vim, but it helps when debugging and is trivially lightweight
# Pip is for some reason massive (300mb) unless you don't install recommended
RUN apt-get update && apt-get install -y \
    gcc \
    git \
    libicu70 \
    python3 \
    vim \
    curl && \
    apt-get install -y --no-install-recommends \
    python3-pip

# This isn't great, as it assumes that the host already has a built toolchain
# If this isn't true then you can take the approach here: 
# https://github.com/guticketa/riscv-gnu-toolchain-docker/blob/master/Dockerfile
# or download from here: 
# https://github.com/stnolting/riscv-gcc-prebuilt
# WORKDIR /usr/local/bin
# COPY ./riscv32-unknown-elf-gcc /usr/local/bin

WORKDIR /root
RUN pip3 install git+https://gitlab.com/incoresemi/riscof.git

# Rust stuff
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh &&\
    sh rustup.sh -y --profile minimal &&\
    source "$HOME/.cargo/env" &&\
    rustup target add riscv32i-unknown-none-elf &&\
    rustup component add llvm-tools-preview &&\
    cargo install cargo-binutils &&\
    rm -rf /root/.cargo/registry &&\
    rustup target remove x86_64-unknown-linux-gnu

# Override default entrypoint/cmd from base image
ENTRYPOINT ["/usr/bin/env"]
CMD ["/bin/bash"]

