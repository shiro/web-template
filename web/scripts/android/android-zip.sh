#!/usr/bin/env sh
set -e

output_dir="../android/build_android"
web_dir="`pwd`"

cd "$output_dir"
zip -r "$web_dir/.output/public/_build/app.zip" . > /dev/null
