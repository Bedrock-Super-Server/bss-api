use serde::Serialize;

/**
A crafting table recipe.

```
use bss_api::prelude::*;

// Two sticks stacked vertically make a torch... of a custom kind.
let torch = RecipeDef::shaped(1, 2, &["minecraft:coal", "minecraft:stick"], "shop:torch", 4);
// Any arrangement of these two items makes the result.
let dye = RecipeDef::shapeless(&["shop:ruby", "minecraft:bone_meal"], "shop:ruby_dye", 2);
assert_eq!(torch.inputs.len(), 2);
assert_eq!(dye.width, 0);
```
*/
#[derive(Clone, Serialize)]
pub struct RecipeDef {
    /// Width of the pattern, or `0` for a shapeless recipe.
    pub width: i32,
    /// Height of the pattern, or `0` for a shapeless recipe.
    pub height: i32,
    /// Item ids of the pattern, row by row; an empty string leaves a cell empty.
    pub inputs: Vec<String>,
    /// Item id of the result.
    pub output: String,
    /// Number of items crafted.
    pub count: u16,
}

impl RecipeDef {
    /// A recipe whose `inputs` can be placed anywhere in the grid.
    pub fn shapeless(inputs: &[&str], output: impl Into<String>, count: u16) -> Self {
        Self {
            width: 0,
            height: 0,
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
            output: output.into(),
            count,
        }
    }

    /// A recipe whose `inputs` form a `width` by `height` pattern, row by row.
    pub fn shaped(
        width: i32,
        height: i32,
        inputs: &[&str],
        output: impl Into<String>,
        count: u16,
    ) -> Self {
        Self {
            width,
            height,
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
            output: output.into(),
            count,
        }
    }
}
