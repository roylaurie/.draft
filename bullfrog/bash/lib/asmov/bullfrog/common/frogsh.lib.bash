#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

frogsh_mkdir () {
    mkdir -p "$1"
}

frogsh_sudo_mkdir () {
    sudo mkdir -p "$1"
}

frogsh_rmdir () {
    rmdir "$1"
}

frogsh_sudo_rmdir () {
    sudo rmdir "$1"
}

frogsh_rmdir_recurse () {
    rm -rf "$1"
}

frogsh_sudo_rmdir_recurse () {
    sudo rm -rf "$1"
}

frogsh_rm () {
    rmdir -f "$1"
}

frogsh_sudo_rm () {
    sudo rm -f "$1"
}

frogsh_rm_recurse () {
    rm -rf "$1"
}

frogsh_sudo_rm_recurse () {
    sudo rm -rf "$1"
}

frogsh_cd () {
    cd "$1"
}

#frogsh_sudo_cd () {
#    cd "$1"
#}

frogsh_chmod () {
    chmod "$1" "$2"
}

frogsh_sudo_chmod () {
    sudo chmod "$1" "$2"
}

frogsh_chmod_recurse () {
    chmod -R "$1" "$2"
}

frogsh_sudo_chmod_recurse () {
    chmod -R "$1" "$2"
}

frogsh_sudo_chown () {
    chown "$1":"$2" "$3"
}

frogsh_chown_recurse () {
    chown -R "$1":"$2" "$3"
}

frogsh_sudo_chown_recurse () {
    sudo chown -R "$1":"$2" "$3"
}

##
# chowns and chmods a file
#
# @param string _user
# @param string _group
# @param string _permissions
# @param string _file
##
frogsh_ch () {
    frogsh_chown "$1" "$2" "$4"
    frogsh_cmod "$3" "$4"
}

frogsh_sudo_ch () {
    frogsh_chown "$1" "$2" "$4"
    frogsh_cmod "$3" "$4"
}

frogsh_cp () {
    cp "$1" "$2"
}

frogsh_sudo_cp () {
    sudo cp "$1" "$2"
}

frogsh_mv () {
    mv "$1" "$2"
}

frogsh_sudo_mv () {
    sudo mv "$1" "$2"
}

