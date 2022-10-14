use image::{
    imageops::{blur, overlay},
    DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba,
};
use imageproc::drawing::draw_filled_circle_mut;
use rusttype::{point, Font, Scale};

use super::draw::{as_glyphs, draw_rounded_rect, draw_text, draw_words, glyphs_width};
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub font: Font<'static>,
    pub scale: Scale,
    pub screen: Size,
    pub sprite: Size,
    pub text: Size,
}

#[derive(Clone, Debug)]
pub struct Renderer {
    pub font: Font<'static>,
    pub scale: Scale,
    pub screen: Size,
    pub text: Size,
}

#[derive(Clone, Debug, Default)]
pub struct RenderOption {}

impl Size {
    pub fn new(xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }
}

impl Renderer {
    pub fn new(font: Font<'static>, scale: Scale, screen: Size, text: Size) -> Self {
        Self {
            font,
            scale,
            screen,
            text,
        }
    }
    pub fn render_dialogue(
        &self,
        bg: Option<&DynamicImage>,
        sprites: &[(&DynamicImage, i64, i64, u8)],
        cname: &str,
        content: &str,
        dialogue_color: [u8; 4],
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let height = v_metrics.ascent - v_metrics.descent;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let mut text_box: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(
            self.screen.xmax,
            self.text.ymax - self.text.ymin + height as u32 + 20,
        );

        if let Some(bg) = bg {
            let resized_bg = bg.resize_exact(
                self.screen.xmax - self.screen.xmin,
                self.screen.ymax - self.screen.ymin,
                image::imageops::FilterType::Nearest,
            );
            overlay(&mut image, &resized_bg, 0, 0);
        }

        for (sprite, x, y, _) in sprites {
            let (width, height) = sprite.dimensions();
            let (width, height) = (width as i64, height as i64);
            let left = (width / 2 - x).max(self.screen.xmin.into());
            let top = (height / 2 - y).max(self.screen.ymin.into());

            let effective_x = x - width / 2;
            let effective_y = *y - height / 2;

            if left > 0 || top > 0 {
                overlay(
                    &mut image,
                    &(*sprite).clone().crop_imm(
                        left as u32,
                        top as u32,
                        width as u32,
                        height as u32,
                    ),
                    effective_x + left as i64,
                    effective_y + top as i64,
                );
            } else {
                overlay(&mut image, *sprite, effective_x as i64, effective_y as i64);
            }
        }

        draw_rounded_rect(
            &mut text_box,
            (0, height as u32 + 20),
            (
                self.text.xmax - self.text.xmin,
                self.text.ymax - self.text.ymin + height as u32,
            ),
            dialogue_color.into(),
            8,
        );

        if !cname.is_empty() {
            let name_width = glyphs_width(&as_glyphs(cname, &self.font, self.scale, point(0., 0.)));

            let height = v_metrics.ascent - v_metrics.descent;
            draw_rounded_rect(
                &mut text_box,
                (0, 0),
                (name_width.max(8 * 2), height as u32 + 30),
                dialogue_color.into(),
                8,
            );

            draw_filled_circle_mut(
                &mut text_box,
                (name_width as i32, height as i32 + 20),
                height as i32 + 20,
                dialogue_color.into(),
            );
        }

        let mut text_box = blur(&text_box, 1.1);

        if !cname.is_empty() {
            draw_text(
                cname,
                &[255, 255, 255, 255].into(),
                &mut text_box,
                &self.font,
                self.scale,
                point(15., height + 5.),
            );
        }

        overlay(
            &mut image,
            &text_box,
            self.text.xmin.into(),
            (self.text.ymin - height as u32 - 20) as i64,
        );

        let mut scale = self.scale;
        let vertical_pad = v_metrics.ascent as u32;

        let ycur = self.text.ymin;
        let mut v_metrics;
        let mut glyphs_height;
        let mut whitespace_width;
        loop {
            v_metrics = self.font.v_metrics(scale);
            glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            whitespace_width = glyphs_width(
                &self
                    .font
                    .layout("_", scale, point(0., 0.))
                    .collect::<Vec<_>>(),
            );
            let (y, _) = content
                .split(' ')
                .map(|word| as_glyphs(word, &self.font, scale, point(self.text.xmin as f32, 0.)))
                .fold(
                    (ycur, self.text.xmin + whitespace_width),
                    |(mut ycur, mut xcur), glyphs| {
                        if !glyphs.is_empty() {
                            let width = glyphs_width(&glyphs);
                            if xcur + width + whitespace_width > self.text.xmax - self.text.xmin {
                                ycur += glyphs_height;
                                xcur = self.text.xmin;
                            }
                            xcur += width + whitespace_width;
                        }

                        (ycur, xcur)
                    },
                );
            if y + vertical_pad + glyphs_height * 2 < self.text.ymax {
                break;
            }
            scale = Scale::uniform(scale.x * 0.95);
        }

        draw_words(
            content,
            &[255, 255, 255, 255].into(),
            &mut image,
            &self.font,
            scale,
            point(
                whitespace_width as f32 + self.text.xmin as f32,
                whitespace_width as f32 + self.text.ymin as f32 + glyphs_height as f32,
            ),
            self.text.xmax - self.text.xmin - whitespace_width,
        );

        image
    }

    pub fn render_choice(
        &self,
        bg: Option<&DynamicImage>,
        choices: &(&str, &str),
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyph_height = v_metrics.ascent - v_metrics.descent;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let white = Rgba::from_slice(&[255, 255, 255, 255]);

        let mut opacity_box: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.screen.xmax, (glyph_height * 1.5) as u32);

        for pixel in opacity_box.pixels_mut() {
            pixel.0 = [0, 0, 0, 255 / 2];
        }

        if let Some(bg) = bg {
            let resized_bg = bg.resize_exact(
                self.screen.xmax - self.screen.xmin,
                self.screen.ymax - self.screen.ymin,
                image::imageops::FilterType::Gaussian,
            );
            overlay(&mut image, &resized_bg, 0, 0);
        }

        let a_glyphs = &as_glyphs(
            choices.0,
            &self.font,
            self.scale,
            point(self.screen.xmax as f32 / 2.0, self.screen.ymax as f32 / 4.0),
        );
        let a_width = glyphs_width(a_glyphs);
        let b_glyphs = &as_glyphs(
            choices.1,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0,
                self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0,
            ),
        );
        let b_width = glyphs_width(b_glyphs);

        overlay(
            &mut image,
            &opacity_box,
            0,
            (self.screen.ymax as f32 / 4.0 - glyph_height) as i64,
        );

        draw_text(
            choices.0,
            white,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0 - a_width as f32 / 2.0,
                self.screen.ymax as f32 / 4.0,
            ),
        );
        overlay(
            &mut image,
            &opacity_box,
            0,
            (self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0 - glyph_height) as i64,
        );

        draw_text(
            choices.1,
            white,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0 - b_width as f32 / 2.0,
                self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0,
            ),
        );

        image
    }
}
