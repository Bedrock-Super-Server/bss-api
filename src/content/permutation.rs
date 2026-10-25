use serde::{Deserialize, Serialize};

/// A value a [`Condition`] compares a state with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateValue {
    /// A word.
    Str(String),
    /// A whole number.
    Int(i32),
    /// A boolean.
    Bool(bool),
}

impl From<&str> for StateValue {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

impl From<String> for StateValue {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<i32> for StateValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<bool> for StateValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl StateValue {
    /// The value as text, the way the block palette writes it: a boolean is `0` or `1`.
    pub(crate) fn text(&self) -> String {
        match self {
            Self::Str(value) => value.clone(),
            Self::Int(value) => value.to_string(),
            Self::Bool(value) => u8::from(*value).to_string(),
        }
    }

    fn molang(&self) -> String {
        match self {
            Self::Str(value) => format!("'{}'", escape(value)),
            Self::Int(value) => value.to_string(),
            Self::Bool(value) => u8::from(*value).to_string(),
        }
    }
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('\'', "\\'")
}

/**
When a [`BlockPermutation`] applies: an expression in Molang, the language the game uses for it, that the client
evaluates for each block.

Build the usual ones with [`state_is`](Condition::state_is) and join them with [`and`](Condition::and),
[`or`](Condition::or) and [`not`](Condition::not); [`raw`](Condition::raw) takes any expression.

# Example

```
use bss_api::prelude::*;

let top = Condition::state_is("minecraft:vertical_half", "top");
assert_eq!(top.as_str(), "q.block_state('minecraft:vertical_half') == 'top'");
let lit_and_high = Condition::state_is("shop:lit", true).and(Condition::state_is("shop:level", 3));
assert_eq!(
    lit_and_high.as_str(),
    "(q.block_state('shop:lit') == 1) && (q.block_state('shop:level') == 3)"
);
```
*/
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Condition(String);

impl Condition {
    /// The state `name` is `value`.
    pub fn state_is(name: &str, value: impl Into<StateValue>) -> Self {
        Self(format!(
            "q.block_state('{}') == {}",
            escape(name),
            value.into().molang()
        ))
    }

    /// The state `name` is not `value`.
    pub fn state_is_not(name: &str, value: impl Into<StateValue>) -> Self {
        Self(format!(
            "q.block_state('{}') != {}",
            escape(name),
            value.into().molang()
        ))
    }

    /// Any Molang expression.
    pub fn raw(expression: impl Into<String>) -> Self {
        Self(expression.into())
    }

    /// Both conditions hold.
    pub fn and(self, other: Condition) -> Self {
        Self(format!("({}) && ({})", self.0, other.0))
    }

    /// One of the two conditions holds.
    pub fn or(self, other: Condition) -> Self {
        Self(format!("({}) || ({})", self.0, other.0))
    }

    /// The condition does not hold.
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self(format!("!({})", self.0))
    }

    /// The expression.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/**
What changes in a custom block for some of its states: when [`condition`](Self::condition) holds, the geometry, the boxes
and the transformation here replace the ones of the block.

Only what is set changes; the rest stays as the block has it. Boxes are in sixteenths of a block, as for
[`CustomBlock::collision`](crate::content::CustomBlock::collision).

# Example

The upper half of a slab: the same shape turned over, with its boxes in the upper half.

```
use bss_api::prelude::*;

let top = BlockPermutation::when(Condition::state_is("minecraft:vertical_half", "top"))
    .collision([-8.0, 8.0, -8.0], [16.0, 8.0, 16.0])
    .selection([-8.0, 8.0, -8.0], [16.0, 8.0, 16.0])
    .rotation([2, 0, 0]);
assert_eq!(top.rotation, Some([2, 0, 0]));
```
*/
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockPermutation {
    /// When it applies.
    pub condition: Condition,
    /// Identifier of another geometry in the resource pack.
    pub geometry: Option<String>,
    /// Another collision box, as `(origin, size)`.
    pub collision: Option<([f32; 3], [f32; 3])>,
    /// Several collision boxes, as `(origin, size)`, for a shape that is not one box such as a stair. When it lists any,
    /// it is used and `collision` is not.
    #[serde(default)]
    pub collisions: Vec<([f32; 3], [f32; 3])>,
    /// Whether whoever walks into the block goes through it: the boxes stay, the collision is turned off, as for an open
    /// fence gate.
    #[serde(default)]
    pub walkthrough: bool,
    /// Another outline, as `(origin, size)`.
    pub selection: Option<([f32; 3], [f32; 3])>,
    /// A move of the shape, in blocks.
    pub translation: Option<[f32; 3]>,
    /// A turn of the shape around each axis, in quarter turns: `[2, 0, 0]` turns it over.
    pub rotation: Option<[i32; 3]>,
}

impl BlockPermutation {
    /// A permutation that applies when `condition` holds and changes nothing yet.
    pub fn when(condition: Condition) -> Self {
        Self {
            condition,
            geometry: None,
            collision: None,
            collisions: Vec::new(),
            walkthrough: false,
            selection: None,
            translation: None,
            rotation: None,
        }
    }

    /// Uses another geometry.
    pub fn geometry(mut self, identifier: impl Into<String>) -> Self {
        self.geometry = Some(identifier.into());
        self
    }

    /// Uses another collision box.
    pub fn collision(mut self, origin: [f32; 3], size: [f32; 3]) -> Self {
        self.collision = Some((origin, size));
        self
    }

    /// Uses several collision boxes, each as `(origin, size)`.
    pub fn collisions(mut self, boxes: impl IntoIterator<Item = ([f32; 3], [f32; 3])>) -> Self {
        self.collisions = boxes.into_iter().collect();
        self
    }

    /// Turns the collision off, so that whoever walks into the block goes through it.
    pub fn walkthrough(mut self) -> Self {
        self.walkthrough = true;
        self
    }

    /// Uses another outline.
    pub fn selection(mut self, origin: [f32; 3], size: [f32; 3]) -> Self {
        self.selection = Some((origin, size));
        self
    }

    /// Moves the shape, in blocks.
    pub fn translation(mut self, by: [f32; 3]) -> Self {
        self.translation = Some(by);
        self
    }

    /// Turns the shape around each axis, in quarter turns.
    pub fn rotation(mut self, quarter_turns: [i32; 3]) -> Self {
        self.rotation = Some(quarter_turns);
        self
    }
}
