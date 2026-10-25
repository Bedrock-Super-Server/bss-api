use serde::{Deserialize, Serialize};

/// The values a [`BlockState`] takes, which also give its type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateValues {
    /// `false` and `true`.
    Bool,
    /// Whole numbers.
    Int(Vec<i32>),
    /// Words.
    Str(Vec<String>),
}

/**
A state of a custom block: a name and the values it takes, such as `minecraft:vertical_half` with `bottom`, `top` and
`double`. The block exists once for each combination of the values of its states, and a [`BlockPermutation`](crate::content::BlockPermutation)
changes how it looks and fills its cell for some of them.

The first value of each state is the one a block has when nothing says otherwise. A block may have up to 8 states,
and up to 256 combinations of values.

# Example

```
use bss_api::prelude::*;

let half = BlockState::strings("minecraft:vertical_half", &["bottom", "top", "double"]);
let level = BlockState::ints("shop:level", 0..=3);
let lit = BlockState::bool("shop:lit");
assert_eq!(half.values, StateValues::Str(vec!["bottom".into(), "top".into(), "double".into()]));
assert_eq!(level.values, StateValues::Int(vec![0, 1, 2, 3]));
assert_eq!(lit.values, StateValues::Bool);
```
*/
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockState {
    /// Name of the state. A name of the game such as `minecraft:vertical_half`, or one of your own with a prefix such as
    /// `shop:level`.
    pub name: String,
    /// What it takes.
    pub values: StateValues,
}

impl BlockState {
    /// A state that is `false` or `true`.
    pub fn bool(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: StateValues::Bool,
        }
    }

    /// A state that takes whole numbers.
    pub fn ints(name: impl Into<String>, values: impl IntoIterator<Item = i32>) -> Self {
        Self {
            name: name.into(),
            values: StateValues::Int(values.into_iter().collect()),
        }
    }

    /// A state that takes words.
    pub fn strings(name: impl Into<String>, values: &[&str]) -> Self {
        Self {
            name: name.into(),
            values: StateValues::Str(values.iter().map(|value| value.to_string()).collect()),
        }
    }
}
