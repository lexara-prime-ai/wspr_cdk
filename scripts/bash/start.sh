#!/bin/bash

printf "\n[Running] <test> container...\n"
sudo docker run -it test python ./hyper/hyper/server.py --interval 5