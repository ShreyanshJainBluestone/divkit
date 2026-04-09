import os
import glob

# Generate mod.rs
files = glob.glob('src/generated/*.rs')
with open('src/generated/mod.rs', 'w') as f:
    for file in files:
        basename = os.path.basename(file)[:-3]
        if basename != 'mod':
            f.write(f'pub mod {basename};\n')

    f.write('\n')
    for file in files:
        basename = os.path.basename(file)[:-3]
        if basename != 'mod':
            f.write(f'pub use {basename}::*;\n')
            
# Also create types module if missing
if not os.path.exists('src/types.rs'):
    with open('src/types.rs', 'w') as f:
        f.write('''
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Expression<T> {
    Value(T),
    Variable(String),
}

impl<T> Expression<T> {
    pub fn value(val: T) -> Self {
        Expression::Value(val)
    }
}
''')
