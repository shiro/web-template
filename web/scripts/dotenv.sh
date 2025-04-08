#!/bin/bash
# loads the environment from .env, but prioritizes env variables

parse_env() {
  [ ! -f "$1" ] && return 0

  while read -r line
  do
    # ignore blanks and comments
    if [[ "$line" =~ ^[[:space:]]*#.* ]] || [[ ! "$line" =~ [^[:space:]] ]]; then
      continue
    fi

    key=`echo "$line" | cut -d'=' -f1 | xargs`
    value=`echo "$line" | cut -d'=' -f2 | cut -d' ' -f1 | xargs`

    eval "current=\${$key}"

    # don't override existing variables
    [ -n "$current" ] && continue

    export "$key"="$value"
  done < "$1"
}

# if [[ "$NODE_ENV" == "production" ]]; then
#   parse_env "../.env.production"
#   parse_env "../.env.production.defaults"
# fi

current_dir=$(realpath "$(pwd)")
while [ "$current_dir" != "/" ]; do
  matched=0

  for arg in "$@"; do
    [ -f "$current_dir/.env.$arg" ]  && parse_env "$current_dir/.env.$arg" && matched=1
    [ -f "$current_dir/.env.$arg.defaults" ] && parse_env "$current_dir/.env.$arg.defaults" && matched=1
  done

  [ -f "$current_dir/.env" ]  && parse_env "$current_dir/.env" && matched=1
  [ -f "$current_dir/.env.defaults" ] && parse_env "$current_dir/.env.defaults" && matched=1

  [ "$matched" -eq 1 ] && return
  
  current_dir=$(dirname "$current_dir")
done
