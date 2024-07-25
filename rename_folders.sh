#!/bin/bash

# Loop through all directories in the current directory
for dir in fyrox*; do
    # Check if the directory name starts with "fyrox-"
    if [[ -d $dir && $dir == fyrox-* ]]; then
        mv "$dir" "${dir/fyrox-/i3m-}"
    # Check if the directory name is exactly "fyrox"
    elif [[ -d $dir && $dir == fyrox ]]; then
        mv "$dir" "i3m"
    fi
done
