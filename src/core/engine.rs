use std::{io, collections::HashMap};

use image::{DynamicImage, imageops::{resize, FilterType::Gaussian}};
use log::{debug, error, info, warn};

use crate::parser::{script::{Script, ScriptContext}, self, directives::Directive};

pub struct Engine {
    script: Script,
    choice: bool,
    active_sprites: Vec<String>,
    // Sprite name -> Sprite img, scale, priority
    sprites: HashMap<String, (DynamicImage, f64, u8)>,
    active_bg: Option<String>,
    bgs: HashMap<String, DynamicImage>, 
}

impl Engine {
    pub fn new(script_path: &str) -> Result<Self, io::Error> {
        Ok (Self { 
            script: Script::new(script_path)?,
            choice: false,
            active_sprites: Vec::new(),
            sprites: HashMap::new(),
            active_bg: None,
            bgs: HashMap::new(),
        })
    }

    pub fn set_choice(&mut self, choice: bool) {
        self.choice = choice;
    }
}

impl Iterator for Engine {
    type Item = Result<ScriptContext, parser::Error>; 
    fn next(&mut self) -> Option<Self::Item> {
        let script = match self.script.next()? {
            Err(e) => return Some(Err(e)),
            Ok(o) => o,
        };

        match script {
            ScriptContext::Dialogue(ref dialogue) => {
                debug!("Parsed dialogue \"{}: {}\"", dialogue.name, dialogue.content);

            },
            // Maybe think of a more modular approach?
            ScriptContext::Directive(ref directive) =>  match directive  {
                Directive::Jump(j) => {
                    if (j.choice_a.is_some() && self.choice) || j.choice_a.is_none(){ // Either conditional jump with choice_a or it's an unconditional jump
                        info!("Jumping to \"{}\"", &j.endpoint);
                        self.script = match Script::new(&j.endpoint) {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Cannot load script: {}", e);
                                panic!()
                            }
                        }
                    }
                },
                Directive::SpriteLoad(sl) => {
                    match image::open(&sl.path) {
                        Ok(dimg) => { 
                            self.sprites.insert(sl.name.clone(), { 
                            match sl.scale {
                                Some(scale) if scale != 1.0 => { 
                                    (resize(&dimg, (dimg.width() as f64 * scale) as u32, (dimg.height() as f64 * scale) as u32, Gaussian).into(), 
                                    scale, 
                                    0) 
                                },
                                _ => (dimg, sl.scale.unwrap_or(1.0), 0), 
                            }});
                            debug!("Loaded sprite \"{}\" from path \"{}\"", sl.name, sl.path)
                        },
                        Err(e) => { 
                            error!("Cannot load image: {}", e);
                            panic!("TODO Handle this later") 
                        }
                    
                }},
                Directive::SpriteHide(sh) => {
                    // TODO: handle the case where the name cannot be found
                    if let Some(idx) = self.active_sprites.iter().position(|n| &sh.name == n) {
                        self.active_sprites.remove(idx);
                        debug!("Removed sprite \"{}\" from active sprites list", sh.name);
                    } else if  self.sprites.iter().any(|(n, ..)| sh.name == **n) {
                        warn!("Sprite \"{}\" exists but cannot be hidden as it already is, consider using `@sprite_show`", sh.name)
                    } else {
                        warn!("Cannot find sprite \"{}\" for removal. Ignoring directive", sh.name);
                    }
                },
                Directive::SpriteShow(ss) => {
                    if  self.active_sprites.iter().any(|n| ss.name == **n) {
                        warn!("Sprite \"{}\" is already active", ss.name)
                    }  else {
                        debug!("Sprite \"{}\" is now active at ({}, {})", ss.name, ss.x, ss.y)
                    }
                },
                Directive::BgLoad(bl) => {
                    match image::open(&bl.path) {
                        Ok(dimg) => { 
                            self.bgs.insert(bl.name.clone(), dimg);
                            debug!("Loaded background \"{}\" from path \"{}\"", bl.name, bl.path)
                         },
                        Err(e) => { error!("Cannot load bg: {}", e); panic!()} ,
                    }
                },
                Directive::BgShow(bs) => {
                    if self.bgs.contains_key(&bs.name) {
                        let old_bg = self.active_bg.replace(bs.name.clone()).unwrap_or_else(|| "<None>".to_string());
                        debug!("Replaced background \"{old_bg}\" with \"{}\"", bs.name)
                    } else {
                        warn!("Cannot find any backgrounds with the name \"{}\". Ignoring directive", bs.name)
                    }
                }
            }
        }
        Some(Ok(script))
    } 
}

mod test {
    #[test]
    fn test() {
        use super::Engine;

        env_logger::init();
        let mut engine = Engine::new("test_script.script").unwrap().peekable();

        while engine.next().is_some() { println!("!") }
    }
}
