#!/usr/bin/env bash
set -e

android_web_dir="../android/build_android"
entrypoint_filenames=`ls .output/public/_build/assets/client-*.js`
entrypoint_css_filenames=`ls .output/public/_build/assets/client-*.css`


# clean
mkdir -p "$android_web_dir"
find "$android_web_dir" -mindepth 1 -maxdepth 1 -exec rm -r {} \;

 # copy
cp -r .output/public/* "$android_web_dir"

body=""
for file in $entrypoint_filenames; do
  body="$body"'\n<script type="module" async="" src="/_build/assets/'"`basename "$file"`"'"></script>'
done

head=""
for file in $entrypoint_css_filenames; do
  head="$head"'\n <link href="/_build/assets/'"`basename "$file"`"'" rel="stylesheet"/>'
done

sed -r \
  -e 's|<!-- body-scripts -->|'"$body"'|' \
  -e 's|<!-- head -->|'"$head"'|' \
  "assets/main.html" > "$android_web_dir/index.html"

# remove unused stuff
find "$android_web_dir" -name '*.highres.*' -exec rm {} \;
find "$android_web_dir" -name '*.gz' -exec rm {} \;
find "$android_web_dir" -name '*.br' -exec rm {} \;
if [ -f "$android_web_dir/app.zip" ]; then
  rm "$android_web_dir/app.zip"
fi
