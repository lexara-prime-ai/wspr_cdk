#!/bin/bash

printf "\n[Inspecting] <test> container...\n"
sudo docker run -it test python ./hyper/hyper/server.py --interval 5

# To do -> Add commands for inspecting resulting image
# docker image inspect [OPTIONS] IMAGE [IMAGE...]