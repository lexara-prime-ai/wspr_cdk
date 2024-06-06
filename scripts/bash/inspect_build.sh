#!/bin/bash

printf "\n[Inspecting] <test> container...\n"
sudo docker run -it test python ./hyper/hyper/server.py --interval 5
