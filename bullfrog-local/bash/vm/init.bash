#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

__script_dir () {
     local _src="${BASH_SOURCE[0]}"
     # while _src is a symlink, resolve it
     while [ -h "$_src" ]; do
          local _dir="$( cd -P "$( dirname "$_src" )" && pwd )"
          local _src="$( readlink "$_src" )"
          # if _src was a relative symlink (no '/' as prefix, resolve it relative to the symlink base directory
          [[ $_src != /* ]] && _src="$_dir/$_src"
     done
     local _dir="$( cd -P "$( dirname "$_src" )" && pwd )"
     echo "$_dir"
}

FROG_ROOT="$(__script_dir)"

# install all required package managers and packages for init
source $FROG_ROOT/bullfrog-local/bash/vm/package/init.bash

source $FROG_ROOT/bullfrog.lib.bash

PARAMS=@(recovery-user vm-type)

# Script parameters
RECOVERY_USER="$(frog_param $PARAMS recovery-user)"
VM_TYPE="$(frog_param $PARAMS vm-type)"

echo
echo -e "${GRN}++---------------------------------------------------------------------------++${NC}"
echo -e "${GRN}+-------------------------- bullfrog / system / init -------------------------+${NC}"
echo -e "${GRN}|                                                                             |${NC}"

# amazon only ...
echo -e "${GRN}|=== Creating recovery user '${RECOVERY_USER}' ...                                                 |${NC}"
frogsys_adduser $RECOVERY_USER sudo,ssh
# end of amazon only

#frog_sshd_init
#frog_user_sshkey $RECOVERY_USER /home/$RECOVERY_USER/bullfrog-recovery.key

echo -e "${GRN}|=== Altering motd ...                                                        |${NC}"
#frog_system_motd_update
frogl_bullet "Altering motd"
frogsh_sudo_cp "$FROG_ROOT/skeleton/local.vm/bullfrog.motd" /etc/update-motd.d
frogsh_sudo_ch root root 644 /etc/update-motd.d/bullfrog.motd
frogsh_sudo_ln /etc/update-motd.d/bullfrog /etc/update-motd.d/00.motd

echo -e "${GRN}|=== Installation complete. Perform the following steps:                      |${NC}"
echo -e "${GRN}|       * Set the maintenance user account's password:   sudo passwd frog     |${NC}"
echo -e "${GRN}|       * Login with SSH.                                                     |${NC}"
echo -e "${GRN}|                                                                             |${NC}"
echo -e "${GRN}+-------------------------- installation complete ----------------------------+${NC}"
echo -e "${GRN}+----------------------- end bullfrog / system / init ------------------------+${NC}"
echo -e "${GRN}++---------------------------------------------------------------------------++${NC}"
echo

exit 0
