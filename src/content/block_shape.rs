use serde::{Deserialize, Serialize};

/**
A form the server knows how to place and break for a custom block, so that a plugin does not write the states, the
permutations and the placing of it by hand.

A plugin that wants something the shapes do not give builds it from [`CustomBlock::states`](crate::content::CustomBlock::states) and
[`CustomBlock::permutations`](crate::content::CustomBlock::permutations), and places the block itself.
*/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BlockShape {
    /// A block of one piece, with the states and permutations the block lists.
    #[default]
    Cube,
    /**
    A slab: the lower half of a block, and nothing else. It is placed as any block is, where the client draws it: never on
    the upper half, and a second one is not stacked into a double slab but goes in the block beside.

    The block gets no state. Its [`geometry`](crate::content::CustomBlock::geometry) is the model of the server for a
    slab unless it gives its own, and its [`collision`](crate::content::CustomBlock::collision) and
    [`selection`](crate::content::CustomBlock::selection) default to the lower half.
    */
    Slab,
    /**
    A stair, which the server places like the stairs of the game: facing the way the player looks, upside down when the
    player clicks the lower half of the side of a block or the underside of one, and with the inner and outer corners
    that neighbouring stairs make.

    The block gets the states that the client sets itself when it places the stair, so that it draws it as the server
    places it: `minecraft:cardinal_direction` (`north`, `south`, `east`, `west`, the side of the tall part),
    `minecraft:vertical_half` (`top` when upside down) and `minecraft:corner` (`none`, `inner_left`, `inner_right`,
    `outer_left`, `outer_right`, made only with the stairs of the same block), and one permutation for each
    combination, which turns the stair and gives it the boxes of its corner. Its [`geometry`](crate::content::CustomBlock::geometry) is a stair whose tall part is on the north,
    lower half down; the server turns it for the other sides and over for the upside down ones.
    */
    Stairs,
    /**
    A trapdoor, which the server places and opens like the trapdoors of the game: facing the way the player looks, on the
    upper half when the player clicks the upper half of the side of a block or the underside of one, and opened and
    closed by a click.

    The block gets the states of the trapdoors of the game: `direction` (`0` east, `1` west, `2` south, `3` north, the
    side of the hinge), `upside_down_bit` and `open_bit`, and one permutation for each combination, which turns the
    trapdoor. Its [`geometry`](crate::content::CustomBlock::geometry) is a closed trapdoor on the lower half, hinged on
    the north; the boxes default to its plate, 3 sixteenths thick.
    */
    Trapdoor,
    /**
    A fence, which links to the fences, the fence gates and the solid blocks beside it, as the fences of the game do, and
    is a block and a half tall for whoever walks into it.

    The block gets the states `minecraft:connection_north`, `east`, `south` and `west`, one permutation for each of the
    sixteen ways it can be linked, with the boxes that go with it, and a model of the server made of a post and the rails
    of the sides it is linked on.
    */
    Fence,
    /**
    A fence gate, which opens and closes with a click and sits lower in a wall.

    The block gets the states `minecraft:cardinal_direction` (the way the player looks, which the client sets itself
    when it places the gate), `in_wall_bit` and `open_bit`, and the model of the server:
    two posts, and the two leaves shut or open. Open, it can be walked through.
    */
    FenceGate,
    /**
    A wall, which links to the walls and the solid blocks beside it, rises to a tall arm under a block above it and stands
    on a post when it bends.

    The block gets the states `wall_post_bit` and `wall_connection_type_north`, `east`, `south` and `west` (`none`,
    `short` or `tall`), the 162 permutations that follow from them, and a model of the server.
    */
    Wall,
    /**
    A pane, thin and linked to the panes, the walls and the solid blocks beside it, as the glass panes and the iron bars of
    the game are.

    The block gets the states `minecraft:connection_north`, `east`, `south` and `west`, the sixteen permutations that
    follow from them, and a model of the server whose bones show the post, the ends and the straight runs.
    */
    Pane,
    /**
    A door, two blocks tall, which the server places as one item, opens and closes with a click, and breaks as a whole.

    The block gets the states `minecraft:cardinal_direction` (which the client sets itself when it places the door),
    `upper_block_bit`, `door_hinge_bit` and `open_bit`, one
    permutation for each of the thirty-two combinations, and a model of the server: a leaf shut against one side or open
    against the other. The same texture covers both halves.
    */
    Door,
}
