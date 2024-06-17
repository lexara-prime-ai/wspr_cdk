#!/bin/bash

printf "\n[Running] <test> container...\n"
sudo docker run -it -p 8000:8000 -e GOOGLE_APPLICATION_CREDENTIALS=/service_account.json -v ./service_account.json:/wspr_cdk/service_account.json wspr_cdk python ./hyper/hyper/server.py --interval 10