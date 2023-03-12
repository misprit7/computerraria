#!/usr/bin/sh

#################################################
# Script to push docker image to docker hub
#################################################

docker tag computerraria:latest misprit7/computerraria:latest
docker push misprit7/computerraria:latest

