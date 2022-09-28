use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io,
    path::Path,
};

use image::{
    imageops::{resize, FilterType::Gaussian},
    DynamicImage,
};
use log::{debug, error, info, warn};
use rusttype::{Font, Scale};

use crate::{
    parser::{
        self,
        directives::Directive,
        error::Span,
        script::{Script, ScriptContext},
    },
    render::renderer::{Renderer, Size},
};

pub struct Engine {
    script: Script,
    choice: bool,
    // (Sprite name, x, y)
    active_sprites: Vec<(String, u64, u64)>,
    // Sprite name -> Sprite img, scale, priority
    sprites: HashMap<String, (DynamicImage, f64, u8)>,
    active_bg: Option<String>,
    bgs: HashMap<String, DynamicImage>,
    renderer: Renderer,
    // [r, g, b, a]
    dialogue_colors: HashMap<String, [u8; 4]>,
    renderable: Option<Renderable>,
}

impl Engine {
    pub fn new(script_path: &str) -> Result<Self, io::Error> {
        let font_data = include_bytes!("../../assets/fonts/calibri-regular.ttf");
        let font = Font::try_from_bytes(font_data).unwrap();

        Ok(Self {
            script: Script::new(script_path)?,
            choice: false,
            active_sprites: Vec::new(),
            sprites: HashMap::new(),
            active_bg: None,
            bgs: HashMap::new(),
            renderer: Renderer::new(
                font,
                Scale::uniform(24.),
                Size::new(0, 640, 0, 480),
                Size::new(20, 620, 340, 480),
            ),
            dialogue_colors: HashMap::new(),
            renderable: None,
        })
    }

    pub fn set_choice(&mut self, choice: bool) {
        self.choice = choice;
    }

    pub fn render<P: AsRef<Path>>(&mut self, folder_path: P) -> Option<u64> {
        let renderable = self.renderable.as_ref()?;
        let hash = self.render_hash();

        let path = folder_path
            .as_ref()
            .join(hash.to_string())
            .with_extension("png");

        if path.exists() {
            info!(
                "Found identical rendered result for hash {}, stopping rendering",
                hash
            );
            return Some(hash);
        }

        let bg = self.active_bg.clone().map(|bg| self.bgs.get(&bg).unwrap());

        let mut sprites = self
            .active_sprites
            .iter()
            .map(|(name, x, y)| {
                let (sprite_img, _, priority) = self.sprites.get(name).unwrap();
                (sprite_img, *x as i64, *y as i64, *priority)
            })
            .collect::<Vec<_>>();
        sprites.sort_by(|a, b| a.2.cmp(&b.2));

        let image = match renderable {
            Renderable::Dialogue(dialogue) => {
                let dialogue_color =
                    self.dialogue_colors
                        .get(&dialogue.name)
                        .unwrap_or(&[0, 0, 0, 255 / 2]);
                self.renderer.render_dialogue(
                    bg,
                    &sprites,
                    &dialogue.name,
                    &dialogue.content,
                    *dialogue_color,
                )
            }
            Renderable::Choice(choice) => self
                .renderer
                .render_choice(bg, &(&choice.choices.0, &choice.choices.1)),
        };

        image.save(path).unwrap();

        Some(hash)
    }

    pub fn render_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.renderable.hash(&mut hasher);

        if let Some(Renderable::Dialogue(_)) = &self.renderable {
            self.active_sprites.iter().for_each(|(name, ..)| {
                let (_, scale, _) = self.sprites.get(name).unwrap();
                ((scale * 1000.) as u64).hash(&mut hasher)
            });
        }
        hasher.finish()
    }
}

#[derive(Hash, Debug)]
enum Renderable {
    Dialogue(Dialogue),
    Choice(Choice),
}

#[derive(Hash, Debug)]
struct Dialogue {
    bg: Option<String>,
    name: String,
    content: String,
    sprites: Vec<(String, u64, u64)>,
    dialogue_color: [u8; 4],
}

#[derive(Hash, Debug)]
struct Choice {
    bg: Option<String>,
    choices: (String, String),
    sprites: Vec<(String, u64, u64)>,
}

impl Iterator for Engine {
    type Item = Result<ScriptContext, parser::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let script = match self.script.next()? {
            Err(e) => {
                error!("{}", e.cause);
                return Some(Err(e));
            }
            Ok(o) => o,
        };
        self.renderable = None;
        match script {
            ScriptContext::Dialogue(ref dialogue) => {
                debug!(
                    "Parsed dialogue \"{}: {}\"",
                    dialogue.name, dialogue.content
                );

                self.renderable = Some(Renderable::Dialogue(Dialogue {
                    bg: self.active_bg.clone(),
                    name: dialogue.name.clone(),
                    content: dialogue.content.clone(),
                    dialogue_color: *self
                        .dialogue_colors
                        .get(&dialogue.name)
                        .unwrap_or(&[0, 0, 0, 0]),
                    sprites: self.active_sprites.clone(),
                }));
            }
            // Maybe think of a more modular approach?
            ScriptContext::Directive(ref directive) => {
                match directive {
                    Directive::Jump(j) => {
                        if j.choice_a.is_some() {
                            self.renderable = Some(Renderable::Choice(Choice {
                                bg: self.active_bg.clone(),
                                choices: (j.choice_a.clone().unwrap(), j.choice_b.clone().unwrap()),
                                sprites: self.active_sprites.clone(),
                            }));
                        }
                        if (j.choice_a.is_some() && self.choice) || j.choice_a.is_none() {
                            // Either conditional jump with choice_a or it's an unconditional jump
                            info!("Jumping to \"{}\"", &j.endpoint);
                            self.script = match Script::new(&j.endpoint) {
                                Ok(s) => s,
                                Err(e) => {
                                    error!("Cannot load script: {}", e);
                                    return Some(Err(parser::Error::new(
                                        self.script.file(),
                                        Span::new(self.script.line(), 0),
                                        e.into(),
                                    )));
                                }
                            }
                        }
                    }
                    Directive::SpriteLoad(sl) => match image::open(&sl.path) {
                        Ok(dimg) => {
                            self.sprites.insert(sl.name.clone(), {
                                match sl.scale {
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
                                    _ => (dimg, sl.scale.unwrap_or(1.0), 0),
                                }
                            });
                            debug!("Loaded sprite \"{}\" from path \"{}\"", sl.name, sl.path)
                        }
                        Err(e) => {
                            error!("Cannot load image: {}", e);
                            return Some(Err(parser::Error::new(
                                self.script.file(),
                                Span::new(self.script.line(), 0),
                                e.into(),
                            )));
                        }
                    },
                    Directive::SpriteHide(sh) => {
                        // TODO: handle the case where the name cannot be found
                        if let Some(idx) =
                            self.active_sprites.iter().position(|(s, ..)| &sh.name == s)
                        {
                            self.active_sprites.remove(idx);
                            debug!("Removed sprite \"{}\" from active sprites list", sh.name);
                            //info!("current renderable is_some: {}", self.renderable.is_some());
                        } else if self.sprites.iter().any(|(n, ..)| sh.name == **n) {
                            warn!("Sprite \"{}\" exists but cannot be hidden as it already is, consider using `@sprite_show`", sh.name)
                        } else {
                            warn!(
                                "Cannot find sprite \"{}\" for removal. Ignoring directive",
                                sh.name
                            );
                        }
                    }
                    Directive::SpriteShow(ss) => {
                        if let Some(sprite) =
                            self.active_sprites.iter_mut().find(|(s, ..)| &ss.name == s)
                        {
                            sprite.1 = ss.x;
                            sprite.2 = ss.y;
                        } else {
                            debug!(
                                "Sprite \"{}\" is now active at ({}, {})",
                                ss.name, ss.x, ss.y
                            );
                            self.active_sprites.push((ss.name.clone(), ss.x, ss.y));
                        }
                    }
                    Directive::BgLoad(bl) => match image::open(&bl.path) {
                        Ok(dimg) => {
                            self.bgs.insert(bl.name.clone(), dimg);
                            debug!(
                                "Loaded background \"{}\" from path \"{}\"",
                                bl.name, bl.path
                            )
                        }
                        Err(e) => {
                            error!("Cannot load bg: {}", e);
                        }
                    },
                    Directive::BgShow(bs) => {
                        if self.bgs.contains_key(&bs.name) {
                            let old_bg = self
                                .active_bg
                                .replace(bs.name.clone())
                                .unwrap_or_else(|| "<None>".to_string());
                            debug!("Replaced background \"{old_bg}\" with \"{}\"", bs.name)
                        } else {
                            warn!("Cannot find any backgrounds with the name \"{}\". Ignoring directive", bs.name)
                        }
                    }
                    Directive::DialogueColor(dc) => {
                        self.dialogue_colors.insert(
                            dc.name.clone(),
                            [dc.red, dc.green, dc.blue, (255. * dc.alpha) as u8],
                        );
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
        let mut engine = Engine::new("test_script.script").unwrap();

        while engine.next().is_some() {
            engine.render("assets/rendered");
        }
    }
}
