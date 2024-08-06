#!/bin/bash

# Define the crates in the correct order of dependency
CRATES=("i3m-core" "i3m-animation" "i3m-resource" "i3m-ui" "i3m-scripts" "i3m-graph" "i3m-impl" "i3m-dylib" "i3m-sound" "i3m")

for crate in "${CRATES[@]}"; do
  echo "Publishing $crate..."
  cd $crate

  # Get the version defined in Cargo.toml
  version=$(grep '^version =' Cargo.toml | awk '{print $3}' | tr -d '"')

  # Check if the crate with the same version is already published
  if cargo info $crate | grep -q "$crate@$version"; then
    echo "$crate version $version is already published. Skipping."
    cd ..
    continue
  fi

  # Attempt to publish
  cargo publish --allow-dirty
  if [ $? -ne 0 ]; then
    echo "Failed to publish $crate. Exiting."
    exit 1
  fi

  cd ..
done

echo "All crates processed."
