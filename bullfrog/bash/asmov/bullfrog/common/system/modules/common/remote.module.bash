#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset

#frog_import "@asmov/bullfrog/bullfrog.lib.bash"

op_common_remote_default () {
    frog_option_remote
}

op_common_remote_install () {
    local _host _pkgFilename _pkgFilepath
    _host="$(frog_option_remote)"
    _pkgFilename="bullfrog-$(frog_version)-amd64.deb"
    _pkgFilepath="$(realpath "$(frog_common_path)/files/debian-package/$_pkgFilename")" ||
        frog_error 1 "Debian package file does not exist"

    [[ "$_host" = "localhost" ]] && frog_error 1 "-r[emote host] not set"

    ssh "$_host" 'mkdir -p ~/tmp'
    scp "$_pkgFilepath" "${_host}:~/tmp" 1> /dev/null
    # shellcheck disable=SC2029
    ssh "$_host" "sudo dpkg -i ~/tmp/$_pkgFilename && bullfrog common.sysinfo"
}