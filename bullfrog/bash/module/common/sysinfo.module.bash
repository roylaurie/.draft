#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset


op_common_sysinfo_default () {
    local -a _arr _arr2
    local _str _str2
    local -A _vals=()

    _vals[os.name]="$(grep -e '^PRETTY_NAME=' /etc/os-release | sed -r -e "s/^.+=['\"]?//g" -e "s/['\"]?$//g")"

    _str="$(grep -e '^ID=' /etc/os-release | sed -r -e "s/^.+=['\"]?//g" -e "s/['\"]?$//g")"
    _str2="$(grep -e '^ID_LIKE=' /etc/os-release | sed -r -e "s/^.+=['\"]?//g" -e "s/['\"]?$//g")"
    case "$_str2" in
        "ubuntu") _vals[os.heritage]="$_str :: $_str2 :: debian" ;;
        *) _vals[os.heritage]="$_str" ;;
    esac

    _str="$(uname -r)"
    _vals[os.kernel]="${_str/[!0-9.]*}"

    _vals[os.arch]="$(uname -p)"
    _vals[hardware.cpu]="$(grep '^model name' /proc/cpuinfo | head -1 | sed -e 's/^.*: //g')"

    _str="$(grep '^MemTotal' /proc/meminfo | sed -re 's/^.*:\s*//g' -e 's/ kB//g')"
    _vals[hardware.ram]="$(echo "$_str / 1000000" | bc) gb"

    mapfile -t _arr <<< "$(df -h | grep "\/$" | awk '{ print $2 ; print $4 }')"
    _vals[storage.total]="${_arr[0]:0:-1}"
    _vals[storage.free]="${_arr[1]:0:-1}"

    _vals[network.hostname]="$(hostname)"
    _vals[network.ip]="$(dig +short myip.opendns.com @resolver1.opendns.com)"

    local -a _networkInterfaces
    mapfile -t _networkInterfaces <<< \
        "$(ip address show | grep ' scope global ' | awk '{ sub(/\/.*$/, "", $2) ; print $2 " " $NF }')"

    _vals[bash.version]="${BASH_VERSION/%[!0-9.]*}"
    _vals[bash.ostype]="$OSTYPE"
    _vals[bash.extglob]="$(shopt extglob > /dev/null && echo "on" || echo "off")"
    _vals[nodejs.version]="$(command -v node > /dev/null && { _str="$(node -v)" && echo "${_str[@]:1}"; } || echo "not installed")"
    _vals[nodejs.v8.version]="$(command -v node > /dev/null && { node -p process.versions.v8 | sed -e 's/\.[0-9]*\-.*$//'; } || echo "not installed" )"
    _vals[python.version]="$(command -v python3 > /dev/null && { python3 --version | awk '{ print $2 }'; } || echo "not installed" )"
    _vals[rust.version]="$(command -v rustc > /dev/null && { rustc --version | awk '{ print $2 }'; } || echo "not installed")"
    _vals[java.version]="$(command -v java > /dev/null && { java --version | head -1 | awk '{ print $2 }'; } || echo "not installed")"

    frogl_header "common.sysinfo"
    frogl_bullet "OS"
    frogl_print_data "Name" "${_vals[os.name]}"
    frogl_print_data "Heritage" "${_vals[os.heritage]}"
    frogl_print_data "Kernel" "${_vals[os.kernel]}"
    frogl_print_data "Arch" "${_vals[os.arch]}"
    frogl_spacer
    frogl_bullet "Hardware"
    frogl_print_data "CPU" "${_vals[hardware.cpu]}"
    frogl_print_data "RAM" "${_vals[hardware.ram]}"
    frogl_spacer
    frogl_bullet "Storage"
    frogl_print_data "Root" "${_vals[storage.free]} gb (free) / ${_vals[storage.total]} gb (total)"
    frogl_spacer
    frogl_bullet "Network"
    frogl_print_data "Hostname" "${_vals[network.hostname]}"
    frogl_print_data "IP" "${_vals[network.ip]}"
    frogl_print_data "Interfaces" ""

    for _str in "${_networkInterfaces[@]}"; do
        read -ra _arr2 <<< "$_str"
        frogl_print_data "   ${_arr2[1]}" "${_arr2[0]}"
    done

    frogl_spacer
    frogl_bullet "bash"
    frogl_print_data "Version" "${_vals[bash.version]}"
    frogl_print_data "OS Type" "${_vals[bash.ostype]}"
    frogl_print_data "extglob" "${_vals[bash.extglob]}"
    frogl_spacer
    frogl_bullet "nodejs"
    frogl_print_data "Version" "${_vals[nodejs.version]}"
    frogl_print_data "V8 Version" "${_vals[nodejs.v8.version]}"
    frogl_spacer
    frogl_bullet "python"
    frogl_print_data "Version" "${_vals[python.version]}"
    frogl_spacer
    frogl_bullet "rust"
    frogl_print_data "Version" "${_vals[rust.version]}"
    frogl_spacer
    frogl_bullet "java"
    frogl_print_data "Version" "${_vals[java.version]}"
    frogl_footer
}