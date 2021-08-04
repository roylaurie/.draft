#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

declare -ar _FROG_COMMON_BUILTIN_PACKAGES=( "bullfrog" "bullfrog-local-vm" "bullfrog-gitwf" )
