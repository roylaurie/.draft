#!/bin/bash
##
# Bullfrog
# Copyright (c) 2021 Asmov LLC
# Licensed under GPLv3
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
##
set -o errexit -o pipefail -o privileged -o nounset
shopt -s extglob

# shellcheck source=./../lib/asmov/bullfrog/common/bullfrog.lib.bash
source "$(realpath "$(dirname "$(realpath "${BASH_SOURCE[0]}")")"/..)"/lib/asmov/bullfrog/common/bullfrog.lib.bash
frog_import_builtins

##
# Calls on the bullfrog common library to parse the command line arguments and run a command.
#
# @params ... cmdline ($@)
# @returns 1: error, 0: success
##
main () {
    local -A _options _namespace _operation _parameters
    frog_read_cmdline _options _namespace _operation _parameters "$@"
    frog_process_options _options
    frog_load_user
    frog_run_operation "$_namespace" "$_operation" _parameters
}

main "$@"
