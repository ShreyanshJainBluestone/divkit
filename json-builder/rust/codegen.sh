#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
cd $SCRIPT_DIR/../../api_generator/

# Run the python generator
PYTHONPATH=. python3 -m api_generator -c ../json-builder/rust/generator-config.json -s ../schema -o ../json-builder/rust/src/generated

cd ../json-builder/rust/src/generated
cat *.rs > _all.tmp
rm *.rs
mv _all.tmp mod.rs
# Add global allow flags at the very top
echo "// Generated code. Do not modify." > temp.rs
echo "#![allow(non_camel_case_types)]" >> temp.rs
echo "#![allow(non_snake_case)]" >> temp.rs
echo "#![allow(dead_code)]" >> temp.rs
echo "use serde::{Serialize, Deserialize};" >> temp.rs
echo "use crate::types::*;" >> temp.rs
cat mod.rs >> temp.rs
mv temp.rs mod.rs
