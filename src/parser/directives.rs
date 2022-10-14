use derive_directive::{directive, DirectiveEnum};
use directive_errors::VerifyError;
use enum_dispatch::enum_dispatch;
use image::imageops::{resize, FilterType::Gaussian};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::{
    core::engine::{Choice, EngineResource, Renderable},
    parser::{self, error::Span, script::Script},
};

macro_rules! null_exec {
    ($directive: ident) => {
        impl Executable for $directive {
            fn execute(&self, _: &mut EngineResource) -> Result<(), crate::parser::Error> {
                Ok(())
            }
        }
    };
}

#[derive(Debug, Clone, Deserialize, Serialize, DirectiveEnum)]
#[enum_dispatch]
pub enum Directive {
    Jump(Jump),
    SpriteLoad(SpriteLoad),
    SpriteHide(SpriteHide),
    SpriteShow(SpriteShow),
    BgLoad(BgLoad),
    BgShow(BgShow),
    DialogueColor(DialogueColor),
    SoundPlay(SoundPlay),
    MusicPlay(MusicPlay),
    Metadata(Metadata),
}

#[enum_dispatch(Directive)]
pub trait Executable {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "jump", verify = jump_verify)]
pub struct Jump {
    endpoint: String,
    choice_a: Option<String>,
    choice_b: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "sprite_load")]
pub struct SpriteLoad {
    name: String,
    path: String,
    scale: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "sprite_hide")]
pub struct SpriteHide {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "sprite_show")]
pub struct SpriteShow {
    name: String,
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "sprite_prio")]
pub struct SpritePriority {
    name: String,
    priority: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "bg_load")]
pub struct BgLoad {
    name: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "bg_show")]
pub struct BgShow {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[directive(keyword = "dialogue_color")]
pub struct DialogueColor {
    name: String,
    red: u8,
    green: u8,
    blue: u8,
    alpha: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[directive(keyword = "sound_play")]
pub struct SoundPlay {
    path: String,
    volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[directive(keyword = "music_play")]
pub struct MusicPlay {
    path: String,
    volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[directive(keyword = "metadata")]
pub struct Metadata {
    key: String,
    value: String,
}

// #[derive(Debug, Clone)]
// #[directive(keyword = "bg_hide")]
// pub struct BgHide {
// }

fn jump_verify(jump: &Jump) -> Result<(), VerifyError> {
    if jump.choice_a.is_some() && jump.choice_b.is_none() {
        return Err(VerifyError::Custom(
            "choices must be both set or not set at all".to_string(),
        ));
    }

    Ok(())
}

impl Executable for Jump {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), parser::Error> {
        if self.choice_a.is_some() {
            resource.renderable = Some(Renderable::Choice(Choice {
                bg: resource.active_bg.clone(),
                choices: (
                    self.choice_a.clone().unwrap(),
                    self.choice_b.clone().unwrap(),
                ),
                sprites: resource.active_sprites.clone(),
            }));
        }
        if (self.choice_a.is_some() && resource.choice) || self.choice_a.is_none() {
            // Either conditional jump with choice_a or it's an unconditional jump
            info!("Jumping to \"{}\"", &self.endpoint);
            resource.script = match Script::new(&self.endpoint) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Cannot load script: {}", e);
                    return Err(parser::Error::new(
                        resource.script.file(),
                        Span::new(resource.script.line(), 0),
                        e.into(),
                    ));
                }
            };
        };
        Ok(())
    }
}

impl Executable for SpriteLoad {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error> {
        match image::open(&self.path) {
            Ok(dimg) => {
                resource.sprites.insert(self.name.clone(), {
                    match self.scale {
                        Some(scale) if scale != 1.0 => (
                            resize(
                                &dimg,
                                (dimg.width() as f64 * scale) as u32,
                                (dimg.height() as f64 * scale) as u32,
                                Gaussian,
                            )
                            .into(),
                            scale,
                            0,
                        ),
                        _ => (dimg, self.scale.unwrap_or(1.0), 0),
                    }
                });
                debug!(
                    "Loaded sprite \"{}\" from path \"{}\"",
                    self.name, self.path
                );
                Ok(())
            }
            Err(e) => {
                error!("Cannot load image: {}", e);
                Err(parser::Error::new(
                    resource.script.file(),
                    Span::new(resource.script.line(), 0),
                    e.into(),
                ))
            }
        }
    }
}

impl Executable for SpriteHide {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error> {
        if let Some(idx) = resource
            .active_sprites
            .iter()
            .position(|(s, ..)| &self.name == s)
        {
            resource.active_sprites.remove(idx);
            debug!("Removed sprite \"{}\" from active sprites list", self.name);
            //info!("current renderable is_some: {}", resource.renderable.is_some());
        } else if resource.sprites.iter().any(|(n, ..)| self.name == **n) {
            warn!("Sprite \"{}\" exists but cannot be hidden as it already is, consider using `@sprite_show`", self.name)
        } else {
            warn!(
                "Cannot find sprite \"{}\" for removal. Ignoring directive",
                self.name
            );
        };
        Ok(())
    }
}

impl Executable for SpriteShow {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error> {
        if let Some(sprite) = resource
            .active_sprites
            .iter_mut()
            .find(|(s, ..)| &self.name == s)
        {
            sprite.1 = self.x;
            sprite.2 = self.y;
        } else {
            debug!(
                "Sprite \"{}\" is now active at ({}, {})",
                self.name, self.x, self.y
            );
            resource
                .active_sprites
                .push((self.name.clone(), self.x, self.y));
        };
        Ok(())
    }
}

impl Executable for BgLoad {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error> {
        match image::open(&self.path) {
            Ok(dimg) => {
                resource.bgs.insert(self.name.clone(), dimg);
                debug!(
                    "Loaded background \"{}\" from path \"{}\"",
                    self.name, self.path
                )
            }
            Err(e) => {
                error!("Cannot load bg: {}", e);
            }
        }
        Ok(())
    }
}

impl Executable for BgShow {
    fn execute(&self, resource: &mut EngineResource) -> Result<(), crate::parser::Error> {
        if resource.bgs.contains_key(&self.name) {
            let old_bg = resource
                .active_bg
                .replace(self.name.clone())
                .unwrap_or_else(|| "<None>".to_string());
            debug!("Replaced background \"{old_bg}\" with \"{}\"", self.name)
        } else {
            warn!(
                "Cannot find any backgrounds with the name \"{}\". Ignoring directive",
                self.name
            )
        }
        Ok(())
    }
}

null_exec!(DialogueColor);
null_exec!(SoundPlay);
null_exec!(MusicPlay);
null_exec!(Metadata);
