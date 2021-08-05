#!/bin/bash
set -o allexport -o errexit -o privileged -o pipefail -o nounset 

frogl_bullet () {
    local _title
    local -i _titleLen
    _title="$1"
    _titleLen="${#_title}"

    printf "$(frog_color green)|=== ${_title}%$((_FROGL_LINE_WIDTH - _titleLen - 6))s|$(frog_color end)\n"
}

frogl_spacer () {
    printf "$(frog_color green)|%$((_FROGL_LINE_WIDTH - 2))s|\n$(frog_color end)"
}

frogl_header () {
    local _title
    local -i _titleLen
    _title="$1"
    _titleLen="${#_title}"

    printf "$(frog_color green)++%$((_FROGL_LINE_WIDTH - 4))s++\n" | tr ' ' '-'

    local -i _len1 _len2
    _len1=$(( (_FROGL_LINE_WIDTH - 2 - _titleLen) / 2 ))
    _len2=$(( (_FROGL_LINE_WIDTH - _len1 - _titleLen - 2) ))
    printf "+%${_len1}s%s%${_len2}s+\n" "" "$_title" "" | tr ' ' '-'

    printf "|%$((_FROGL_LINE_WIDTH - 2))s|\n$(frog_color end)"
}

frogl_footer () {
    local _title
    local -i _titleLen
    _title="${1:-eof}"
    _titleLen="${#_title}"

    printf "$(frog_color green)|%$((_FROGL_LINE_WIDTH - 2))s|\n"

    local -i _len1 _len2
    _len1=$(( (_FROGL_LINE_WIDTH - 2 - _titleLen) / 2 ))
    _len2=$(( (_FROGL_LINE_WIDTH - _len1 - _titleLen - 2) ))
    printf "+%${_len1}s%s%${_len2}s+\n" "" "$_title" "" | tr ' ' '-'

    printf "++%$((_FROGL_LINE_WIDTH - 4))s++\n$(frog_color end)" | tr ' ' '-'
}

frogl_print () {
    local _txt
    local -i _txtLen
    _txt="$1"
    _txtLen="${#_txt}"

    printf "$(frog_color green)|$(frog_color end) ${_txt}%$((_FROGL_LINE_WIDTH - _txtLen - 3))s$(frog_color green)|$(frog_color end)\n"
}

frogl_print_data () {
    local _title _txt
    local -i _titleLen _txtLen _len
    _title="$1"
    _titleLen="${#_title}"
    _txt="$2"
    _txtLen="${#_txt}"
    _len=$((_titleLen + _txtLen))

    printf "$(frog_color green)|$(frog_color lightgray) ${_title}: $(frog_color end)${_txt}%$((_FROGL_LINE_WIDTH - _len - 5))s$(frog_color green)|$(frog_color end)\n"
}

declare -ir _FROGL_LINE_WIDTH=84

