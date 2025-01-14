#!/usr/bin/env bash
# Exposes all files in src/models/enums directory in src/models/enums/mod.rs

cd ../src/models/enums

rm mod.rs

echo "// Auto-generated mod.rs file" > mod.rs

for file in $(ls); do
  if [[ $file != "mod.rs" ]]; then
    echo "pub mod ${file%.*};" >> mod.rs;
  fi
done
