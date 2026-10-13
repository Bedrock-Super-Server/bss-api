use serde::Serialize;

use super::tool::ToolType;

#[derive(Serialize)]
pub(crate) struct BlockDef {
    pub id: String,
    pub display: String,
    pub texture: String,
    pub faces: Vec<(String, String)>,
    pub geometry: String,
    pub collision: Option<([f32; 3], [f32; 3])>,
    pub selection: Option<([f32; 3], [f32; 3])>,
    pub break_ticks: i32,
    pub tool: Option<ToolType>,
    pub slab: bool,
}
