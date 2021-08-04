#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

sudo apt-get update
sudo apt-get upgrade -y
 
#sudo apt-get -y install \

sudo apt-get update
sudo apt-get upgrade -y

sudo snap install node --channel=16/stable --classic

sudo npm install -g \
    ajv-cli
