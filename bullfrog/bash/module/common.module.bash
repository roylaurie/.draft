#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset 

op_common_version () {
    local -n parameters="$1"

    #TODO version extraction
    echo -e "$(frog_color lightgreen)bullfrog $(frog_color lightgray)v$(frog_version)$(frog_color end)"
}