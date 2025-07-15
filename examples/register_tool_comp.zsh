#!/usr/bin/env zsh

autoload -U compinit
compinit

if [[ -d "/Users/andrew" ]]; then
    export PATH=$PATH:/Users/andrew/projects/register_tool/target/debug:/Users/andrew/projects/register_tool/tools/ucompleter/target/debug
elif [[ -d "/home/andrew/" ]]; then
    export PATH=$PATH:/home/andrew/black/register_tool/target/debug:/home/andrew/black/register_tool/tools/ucompleter/target/debug
else
  echo "No project found"
fi

# zstyle ':completion:*:register_tool:*' add-space false

_register_tool_complete() {
#    local curcontext="$curcontext" state line
#    typeset -A opt_args
#
#    _arguments -C \
#        '-f[File of reg definitions]:config file:_files' \
#        '-v[Verbose mode]' \
#        '-d[Dump register properties]' \
#        '*:registers:{ compadd "${(@f)$(ucompleter "$words[1]" "${words[2,-1]}")}" }'
  local -a regs descriptions elements
  # array=( "${(@f)"$(command)"}" )
  # regs=("${(@f)$(ucompleter -z register_tool GPIO.pins)}")
  # regs=(a b c)
  zstyle ':completion:*' add-space false
  elements=( "${(@f)"$(ucompleter -z register_tool $words[2])"}" )
  regs=()
  descriptions=()
  if [[ "${elements[1]}" == "__descriptions__" ]]; then
    for ((i=2; i<=${#elements}; i+=2)); do
      regs+=("${elements[i]}")
      descriptions+=("${elements[i+1]}")
    done
    compadd -d descriptions -a regs
  else
    regs=("${elements[@]}")
    compadd -a regs
  fi

}

_register_tool_complete2() {
  local -a regs
  zstyle ':completion:*' add-space false
  regs=( "${(@f)"$(ucompleter -z register_tool $words[2])"}" )
  compadd -a regs
}


compdef _register_tool_complete2 register_tool


myfunc() {
    local -a regs descriptions elements
  zstyle ':completion:*' add-space false
  elements=( "${(@f)"$(ucompleter -z register_tool GPIO.pins@27.)"}" )
  regs=()
  descriptions=()
  if [[ "${elements[1]}" == "__descriptions__" ]]; then
    for ((i=2; i<=${#elements}; i+=2)); do
      regs+=("${elements[i]}")
      descriptions+=("${elements[i+1]}")
    done
    echo "## completions"
    for ((i=1; i<=${#regs}; i+=1)); do
      echo $regs[i]
    done
    echo "## descriptions"
    echo $descriptions

  else
    regs=("${elements[@]}")
    compadd -a regs
  fi
}

