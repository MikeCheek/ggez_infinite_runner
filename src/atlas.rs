use std::fs::File;
use std::io::BufReader;
use ggez::graphics::{self, Rect};
use serde::{Deserialize};
use std::path::Path;
use ggez::mint::{Point2, Vector2};

#[derive(Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}
#[derive(Deserialize, Debug)]
struct Meta {
    size: AtlasSize,
}

#[derive(Deserialize, Debug, Clone)]
struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct SpriteData {
    filename: String,
    frame: JsonRect,
}

#[derive(Deserialize, Debug)]
pub struct Atlas {
    frames: Vec<SpriteData>,
    meta: Meta,
}

impl Atlas {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        let file = File::open(texture_atlas_file).expect("Couldn't find the texture_atlas file");
        let buf_reader = BufReader::new(file);
        serde_json::from_reader(buf_reader).expect("Couldn't create texture atlas")
    }

    pub fn create_sprite(&self, sprite_name: &str) -> Sprite {
        let width = self.meta.size.w as f32;
        let height = self.meta.size.h as f32;
        let atlas_rect = Rect::new(0.0, 0.0, width, height);

        if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name) {
            Sprite::new(
                Rect::fraction(
                    sprite_data.frame.x as f32,
                    sprite_data.frame.y as f32,
                    sprite_data.frame.w as f32,
                    sprite_data.frame.h as f32,
                    &atlas_rect,
                ),
                sprite_data.frame.w as f32,
                sprite_data.frame.h as f32,
            )
        } else {
            unimplemented!("Not handling failure to find sprite");
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    /// The square that we want to cut out of the texture atlas.
    pub rect: Rect,
    pub scale: Vector2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Sprite {
    pub fn new(rect: Rect, width: f32, height: f32) -> Self {
        Self {
            rect,
            scale: Vector2 { x:1.0,y:1.0 },
            width,
            height,
        }
    }

    /// Adds a draw command to the sprite batch.
    pub fn add_draw_param(&mut self, pos: Point2<f32>) -> graphics::DrawParam {
        self.draw_params(pos)
    }

    pub fn draw_params(&self, pos: Point2<f32>) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .src(self.rect.clone())
            .scale(self.scale)
            .dest(pos)
    }

    /// Returns the bounding box for this sprite.
    pub fn get_bound_box(&self) -> Rect {
        let mut r = Rect::new(0.0, 0.0, self.width, self.height);
        r.scale(self.scale.x, self.scale.y);
        r
    }
}