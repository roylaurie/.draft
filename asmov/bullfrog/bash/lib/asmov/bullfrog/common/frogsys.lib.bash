#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

frogsys_adduser () {
    local _username _groups
    _username="$1"
    _groups="$2"

    sudo adduser --disabled-password --gecos "" "$_username"

    frogsys_usermod_groups "$_groups"
    frogsh_sudo_chown_recurse "$_username" "$_username" "/home/$_username"
    frogsh_sudo_chmod_recurse o-rwx "/home/$_username"
}

frogsys_usermod_set_groups () {
    sudo usermod -G "$1" "$2" 
}

frogsys_usermod_append_groups () {
    sudo usermod -aG "$1" "$2"
}
