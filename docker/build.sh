#/usr/bin/sh
DOCKER_BUILDKIT=1 docker build --secret id=_env,src=./.env -t computerraria .
