#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset

heredocgen_common_sysinfo_default () {
    frogdoc_header "common.sysinfo"
    frogdoc_section "OS"
    frogdoc_field "Name" "os.name"
    frogdoc_field "Heritage" "os.heritage"
    frogdoc_field "Kernel" "os.kernel"
    frogdoc_field "Arch" "os.arch"
    frogdoc_section "Hardware"
    frogdoc_field "CPU" "hardware.cpu"
    frogdoc_field "RAM" "hardware.ram"
    frogdoc_section "Storage"
    frogdoc_field "Root" "%{storage.free}% gb (free) / %{storage.total}% gb (total)"
    frogdoc_section "Network"
    frogdoc_field "Hostname" "network.hostname"
    frogdoc_field "IP" "network.ip"
    frogdoc_fields "network.interfaces" "Interfaces"
    frogdoc_section "bash"
    frogdoc_field "Version" "bash.verison"
    frogdoc_field "OS Type" "bash.ostype"
    frogdoc_field "extglob" "bash.extglob"
    frogdoc_section "nodejs"
    frogdoc_field "Version" "nodejs.verison"
    frogdoc_field "V8 Version" "node.v8.version"
    frogdoc_section "python"
    frogdoc_field "Version" "python.version"
    frogdoc_section "rust"
    frogdoc_field "Version" "rust.version"
    frogdoc_section "java"
    frogdoc_field "Version" "java.version"
    frogdoc_footer
}