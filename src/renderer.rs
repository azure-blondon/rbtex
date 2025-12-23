use image::{ImageBuffer, RgbaImage, Rgba};
use rusttype::{Font, Scale};

use crate::parser::BiOSToken;


fn get_font(path: &str) -> Font<'static> {
    Font::try_from_vec(std::fs::read(path).expect("failed to read font")).expect("invalid font")
}



fn clear_image(img: &mut RgbaImage, bg_color: Rgba<u8>) {
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }
}

fn multiply_color(color: Rgba<u8>, value: f32) -> Rgba<u8> {
    let array: [u8; 4] = color.0;
    Rgba([
        (array[0] as f32 * value).clamp(0., 255.) as u8,
        (array[1] as f32 * value).clamp(0., 255.) as u8,
        (array[2] as f32 * value).clamp(0., 255.) as u8,
        255
    ])
}



pub struct Frame<'a> {
    pub img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub frame_index: usize,
    pub token_index: usize,
    font: Font<'a>,
    scale: Scale,
    pub tokens: Vec<BiOSToken>,
    current_color: Rgba<u8>,
    pub padding_x: i32,
    pub padding_y: i32,
    frames_dir: &'a str,
    v_metrics: rusttype::VMetrics
}


impl<'a> Frame<'a> {

    pub fn new(width: u32, height: u32, font_path: &str, scale: f32, tokens: Vec<BiOSToken>, frames_dir: &'a str) -> Frame<'a>{
        let font: Font<'_> = get_font(font_path);
        let scale: Scale = Scale::uniform(scale);
        Frame {
            img: RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 255])),
            token_index: 0,
            frame_index: 0,
            v_metrics: font.v_metrics(scale),
            font,
            scale,
            tokens,
            current_color: Rgba([255, 255, 255, 255]),
            padding_x: 0,
            padding_y: 0,
            frames_dir,
        }
    }

    pub fn render(&mut self) {
        clear_image(&mut self.img, Rgba([0, 0, 0, 255]));
        self.current_color = Rgba([255, 255, 255, 255]);
        let line_height: i32 = (self.v_metrics.ascent - self.v_metrics.descent + self.v_metrics.line_gap).ceil() as i32;
        let base_y: i32 = self.img.height() as i32 - 2 * self.padding_y - (-self.v_metrics.descent).ceil() as i32;

        let tokens = &self.tokens.clone()[..self.token_index.min(self.tokens.len())];
        let lines: Vec<&[BiOSToken]> = tokens.split(|tok| *tok == BiOSToken::Newline).collect();
        
        for (mut nb_of_lines, line) in lines.iter().enumerate() {
            let mut x: f32 = self.padding_x as f32;
            nb_of_lines = lines.len() - nb_of_lines - 1;
            let y: f32 = (base_y - (nb_of_lines as i32 * line_height)) as f32;
            for token in line.iter() {

                match token {
                    BiOSToken::Char(c) => {
                        x += self.render_text(&c.to_string(), x, y);
                    }
                    BiOSToken::Instant(instant_string) => {
                        x += self.render_text(instant_string, x, y);
                    }
                    BiOSToken::ColorStart(color) => {
                        self.current_color = *color;
                    }
                    BiOSToken::Pause(_) => {}
                    _ => {}
                }
                
            }

        }
        

        self.img.save(format!("{}/frame_{:05}.bmp", self.frames_dir, self.frame_index)).expect("failed to save frame");
    }

    fn render_text(&mut self, text: &String, x: f32, y: f32) -> f32 {
        let mut last_x = x;
        for glyph in self.font.layout(&text, self.scale, rusttype::point(x, y + self.v_metrics.ascent)) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let px = gx as i32 + bb.min.x;
                    let py = gy as i32 + bb.min.y;

                    if px >= 0 && py >= 0 && px < self.img.width() as i32 && py < self.img.height() as i32 {
                        self.img.put_pixel(
                            px as u32,
                            py as u32,
                            multiply_color(self.current_color, v),
                        );
                    }
                });
            }
            last_x = glyph.position().x + glyph.unpositioned().h_metrics().advance_width;
        }
        last_x - x
    }


    pub fn render_next_token(&mut self) {
        if let Some(token) = self.tokens.get(self.token_index) {
            match token {
                BiOSToken::Char(_) | BiOSToken::Instant(_) | BiOSToken::Newline  => {
                    self.render();
                    self.frame_index += 1;
                }
                BiOSToken::Pause(time) => {
                    for _ in 0..*time {
                        self.render();
                        self.frame_index += 1;
                    }
                }
                _ => {}
            }
            self.token_index += 1;
        }
    }

    pub fn render_tokens(&mut self) {
        for _ in 0..self.tokens.len() {
            self.render_next_token();
        }
    }
}
