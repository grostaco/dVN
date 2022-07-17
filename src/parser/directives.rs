use derive_directive::{directive, DirectiveEnum};
use directive_errors::VerifyError;

#[derive(Debug, Clone)]
#[derive(DirectiveEnum)]
pub enum Directive {
    Jump(Jump),
    SpriteLoad(SpriteLoad),
    SpriteHide(SpriteHide),
    SpriteShow(SpriteShow),
    BgLoad(BgLoad),
    BgShow(BgShow),
}

#[derive(Debug, Clone)]
#[directive(keyword = "jump", verify = jump_verify)]
pub struct Jump {
    endpoint: String,
    choice_a: Option<String>,
    choice_b: Option<String>,
}

#[derive(Debug, Clone)]
#[directive(keyword = "sprite_load")]
pub struct SpriteLoad {
    name: String,
    path: String,
    scale: Option<f64>,
}

#[derive(Debug, Clone)]
#[directive(keyword = "sprite_hide")]
pub struct SpriteHide {
    name: String,
}


#[derive(Debug, Clone)]
#[directive(keyword = "sprite_show")]
pub struct SpriteShow {
    name: String,
    x: u64,
    y: u64,
}

#[derive(Debug, Clone)]
#[directive(keyword = "sprite_prio")]
pub struct SpritePriority {
    name: String,
    priority: i64,
}

#[derive(Debug, Clone)]
#[directive(keyword = "bg_load")]
pub struct BgLoad {
    name: String,
    path: String,
}

#[derive(Debug, Clone)]
#[directive(keyword = "bg_show")]
pub struct BgShow {
    name: String,
}

// #[derive(Debug, Clone)]
// #[directive(keyword = "bg_hide")]
// pub struct BgHide {
// }

fn jump_verify(jump: &Jump) -> Result<(), VerifyError>{
    if jump.choice_a.is_some() && jump.choice_b.is_none() {
        return Err(VerifyError::Custom("choices must be both set or not set at all".to_string()));
    }

    Ok(())
}