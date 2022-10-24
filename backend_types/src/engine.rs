use image_rpg::parser::script::ScriptContext;
use serde::{Deserialize, Serialize};

pub type EngineInit = ();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EngineNext {
    Content { id: u64, context: ScriptContext },
    Ended,
}
