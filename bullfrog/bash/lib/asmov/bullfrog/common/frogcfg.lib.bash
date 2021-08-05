#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset

declare -a _FROGCFG_KEYS=()
declare -a _FROGCFG_VALUES=()
declare -a _FROGCFG_OFFSETS=()

##
# Stores a configuration setting in the FrogCFG key-value registry.
# The value type may be either a string or array. Multiple value argurments represent elements within an array.
#
# === Example
# ---
# # Stores a string value
# frogcfg_set_key string "my.str.value" "this is a value"
# ---
# # Stores an entire array
# local -a arr ; arr=("foo bar" "tar zip")
# frog_cfg_set array "my.str.array" "${arr[@]}"
# ---
# # Stores an array, element by element
# frog_cfg_set array "my.str.array" "${arr[0]}" "${arr[1]}"
#
# @param enum("string" "array") valueType
# @param string key
# @param string... value
# @returns 1: error ; 0: success
# @exits 1 Invalid value type
# @exits 1
##
frogcfg_set_key() {
    local _type _key _value
    _type="$1"
    _key="$2"
    shift 2 && _value=("$@")

    [[ "$_type" != "string" && "$_type" != "array" ]] &&
        frog_error 1 "Invalid data type for config key" "$_key" "$_type"

    #TODO prevent duplicate sets against the same key

    local -i _start _size
    _start="${#_FROGCFG_VALUES[@]}"
    _size="${#_value[@]}"

    [[ "$_size" -gt 1 && "$_type" = "string" ]] &&
        frog_error 1 "Cannot convert string to array for config key" "$_key"

    _FROGCFG_KEYS+=("$_key")
    _FROGCFG_VALUES+=("${_value[@]}")
    _FROGCFG_OFFSETS+=("$_type $_start $_size")
}

##
# Returns the FrogCFG key-value array index for the specified key.
#
# @param string key
# @returns 1: key not found ; 0: success { number index }
##
frogcfg_key_index () {
    local _key
    _key="$1"

    for (( i=0, n="${#_FROGCFG_KEYS[@]}"; i < n; ++i )); do
        [[ "${_FROGCFG_KEYS[$i]}" = "$_key" ]] && echo "$i" && return 0
    done

    return 1
}

##
# Determines whether the FrogCFG key-value registry stores the specified key.
#
# @param string key
# @returns 1: not found ; 0: found
##
frogcfg_has_key () {
  frogcfg_key_index "$1" > /dev/null
}

##
# Retrieves a value from the FrogCFG key-value registry.
#
# === Example
# ---
# # Retrieves a string with error handling
# local my1 ; my1="$(frogcfg_get_value "my.str.value")" ||
#     frog_error 1 "Could not find my favorite string"
# ---
# # Retrieves an array string with error handling, then splits the result into an array.
# local str="$(frogcfg_get_value array "my.str.array")" ||
#     frog_error 1 "Could not find my favorite array"
# local -a my2 ; read -ra my2 <<< "$str"
#
# @param enum("string" "array") valueType The expected type of the return value.
# @param string key The config key.
# @param ?string default The default value to return if the key does not exist. Prevents error on key not found.
# @returns 1: error ; 0: success { string value }
# @exits 1: value type mismatch
# @exits 1: key not found
##
frogcfg_read_value() {
    local -n _resultRef
    _resultRef="$1"
    local _type _key _default
    _type="$2"
    _key="$3"
    _default="${4:-}"

    local -i _index
    _index="$(frogcfg_key_index "$_key")" || {
       if [[ -z "${3+null}" ]]; then
            frog_error 1 "Cannot find config key" "$_key"
        else
            echo "$_default"
            return 0
        fi
    }

    local -a _offset
    local _valueType
    local -i _start _size
    read -r -a _offset <<< "${_FROGCFG_OFFSETS[$_index]}"
    _valueType="${_offset[0]}"
    _start="${_offset[1]}"
    _size="${_offset[2]}"

    [[ "$_type" != "$_valueType" ]] &&
        frog_error 1 "Wrong data type specified for config key/type " "$_key / $_type"

    if [[ "$_type" = "array" ]]; then
        _resultRef=("${_FROGCFG_VALUES[@]:$_start:$_size}")
    else
        # shellcheck disable=SC2178
        _resultRef="${_FROGCFG_VALUES[$_start]}"
    fi
}

##
# Retrieves a value from the FrogCFG key-value registry.
#
# === Example
# ---
# # Retrieves a string with error handling
# local my1 ; my1="$(frogcfg_get_value "my.str.value")" ||
#     frog_error 1 "Could not find my favorite string"
# ---
# # Retrieves an array string with error handling, then splits the result into an array.
# local str="$(frogcfg_get_value array "my.str.array")" ||
#     frog_error 1 "Could not find my favorite array"
# local -a my2 ; read -ra my2 <<< "$str"
#
# @param enum("string" "array") valueType The expected type of the return value.
# @param string key The config key.
# @param ?string default The default value to return if the key does not exist. Prevents error on key not found.
# @returns 1: error ; 0: success { string value }
# @exits 1: value type mismatch
# @exits 1: key not found
##
frogcfg_get_value() {
    local _type _key _default
    _type="$1"
    _key="$2"
    _default="${3:-}"

    local -i _index
    _index="$(frogcfg_key_index "$_key")" || {
       if [[ -z "${3+null}" ]]; then
            frog_error 1 "Cannot find config key" "$_key"
        else
            echo "$_default"
            return 0
        fi
    }

    local -a _offset
    local _valueType
    local -i _start _size
    read -r -a _offset <<< "${_FROGCFG_OFFSETS[$_index]}"
    _valueType="${_offset[0]}"
    _start="${_offset[1]}"
    _size="${_offset[2]}"

    [[ "$_type" != "$_valueType" ]] &&
        frog_error 1 "Wrong data type specified for config key/type " "$_key / $_type"

    if [[ "$_type" = "array" ]]; then
        local -a _result
        _result=("${_FROGCFG_VALUES[@]:$_start:$_size}")

        for _r in "${_result[@]}"; do
            echo "$_r"
        done
    else
        echo "${_FROGCFG_VALUES[$_start]}"
    fi
}

##
# Prints all FrogCFG string key-values. Ignored if the frog debug option (-x) is enabled.
#
# @returns 1: error ; 0: success, debugging disabled
##
frogcfg_debug () {
    frog_option_debug || return 0

    for (( _i=0, _n=${#_FROGCFG_KEYS[@]} ; _i < _n ; ++_i )); do
        local _key _valueType
        _key="${_FROGCFG_KEYS[$_i]}" _valueType
        local -a _offset
        read -ra _offset <<< "${_FROGCFG_OFFSETS[$_i]}"
        _valueType="${_offset[0]}"

        [[ "$_valueType" = "string" ]] &&
            frog_debug "CFG" "$_key" "= $(frogcfg_get_value string "$_key")"
    done

    return 0
}
