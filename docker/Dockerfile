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

RUN /root/Steam/steamapps/common/tModLoader/LaunchUtils/InstallNetFramework.sh

# Put apt updates after large terraria install so adding a dependency doesn't require rebuild
# Yeah I'm not supposed to install vim, but it helps when debugging and is trivially lightweight
RUN apt-get update && apt-get install -y \
    libicu70 \
    vim \
    wget


# Override default entrypoint/cmd from base image
ENTRYPOINT ["/usr/bin/env"]
CMD ["/bin/bash"]

