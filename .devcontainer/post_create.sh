#!/bin/bash

set -euxo pipefail
# pipefail

# If set, the return value of a pipeline is the value of the last 
# (rightmost) command to exit with a non-zero status, or zero if 
# all commands in the pipeline exit successfully. 
# This option is disabled by default.

sudo apt-get update
sudo apt-get install -y python3-dev python3-pip python3-venv libclang-dev
sudo python3 -m venv .venv
