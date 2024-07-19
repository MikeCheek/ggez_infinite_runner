mod player;
mod traits;
mod obstacle;
mod constants;
mod atlas;

use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawParam, FontData, Image, InstanceArray, Mesh, PxScale, Text, TextFragment};
use ggez::input::keyboard::KeyCode;
use ggez::mint::{Point2, Vector2};
use crate::atlas::{Atlas, Sprite};
use crate::constants::{GameState, GRAVITY, INCREASE_VELOCITY_EVERY, JUMP_FORCE};
use crate::obstacle::Obstacle;
use crate::player::Player;

pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
    gravity: Vector2<f32>,
    obstacles: Vec<(Obstacle, Mesh)>,
    time_elapsed: f64,
    state: GameState,
    score: u64,
    atlas: Atlas,
    sprite_sheet: Image,
    player_sprites: Vec<Sprite>,
    sprite_index:f32
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let (arena_width, arena_height) = ctx.gfx.drawable_size();
        let player = Player::new();
        let player_mesh = player.create_mesh(ctx)?;
        let gravity = Vector2 { x: 0.0, y: GRAVITY };

        let mut obstacles = Vec::new();
        for i in 0..3 {
            let size = 50.0; //+ (i as f32 *10.0);
            let obst = Obstacle::new(arena_width, arena_height - size, size, i as f32 * 500.0);
            let obst_mesh = obst.create_mesh(ctx)?;
            obstacles.push((obst, obst_mesh));
        }

        ctx.gfx.add_font(
            "Pixel",
            FontData::from_path(ctx, "/PixelColeco.ttf")?,
        );

        let atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/spritesheet.json"));
        // let sprite_batch = Self::create_batch_sprite(ctx);
        let sprite_sheet = Image::from_path(ctx, "/spritesheet.png").unwrap();
        let player_sprites = player.create_sprites(&atlas);

        let mut game = MyGame {
            player,
            player_mesh,
            gravity,
            obstacles,
            time_elapsed: 0.0,
            state: GameState::Started,
            score: 0,
            atlas,
            sprite_sheet,
            player_sprites,
            sprite_index:0.0
        };

        game.reset(ctx);

        Ok(game)
    }

    fn reset(&mut self, ctx: &mut Context) {
        let (arena_width, arena_height) = ctx.gfx.drawable_size();

        self.player.set_location(50.0, 50.0);

        for i in 0..self.obstacles.len() {
            let size = 50.0; //+ (i as f32 *10.0);
            let (obst, _) = self.obstacles.get_mut(i).unwrap();
            obst.set_location(arena_width + (i as f32 * 500.0), arena_height - size)
        }

        self.score = 0;
        self.state = GameState::Started;
    }

    fn create_batch_sprite(ctx: &Context) -> InstanceArray {
        // Load image from path
        let image = Image::from_path(ctx, "/spritesheet.png").unwrap();
        // Set filter mode on image
        // image.set_filter(graphics::FilterMode::Nearest);
        // Create InstanceArray with image
        let batch = InstanceArray::new(ctx, image);
        batch
    }

    pub fn create_text(text: impl Into<TextFragment>, scale: f32) -> Text {
        Text::new(text)
            .set_scale(PxScale::from(scale))
            .set_font("Pixel")
            .to_owned()
    }

    pub fn draw_at_center(ctx: &Context, canvas: &mut Canvas, drawable: Text, offset_y: f32) {
        let (arena_width, arena_height) = ctx.gfx.drawable_size();
        let dimensions = drawable.measure(ctx).unwrap();

        let position = Point2 {
            x: (arena_width - dimensions.x) / 2.0,
            y: ((arena_height - dimensions.y) / 2.0) + offset_y,
        };

        canvas.draw(&drawable, DrawParam::default().dest(position));
    }

    fn draw_game_over_screen(&self, canvas: &mut Canvas, ctx: &Context) {
        let game_over_text = Self::create_text("GAME OVER", 100.0);
        let score_text = Self::create_text(format!("You jumped over {} obstacles", self.score), 50.0);
        let restart_text = Self::create_text("Press SPACE to restart", 30.0);

        let go_dimensions = game_over_text.measure(ctx).unwrap();
        let r_dimensions = game_over_text.measure(ctx).unwrap();

        Self::draw_at_center(ctx, canvas, game_over_text, -40.0);
        Self::draw_at_center(ctx, canvas, score_text, go_dimensions.y - 40.0);
        Self::draw_at_center(ctx, canvas, restart_text, go_dimensions.y + r_dimensions.y - 40.0);
    }

    fn draw_score(&self, canvas: &mut Canvas) {
        let score_text = Self::create_text(format!("Score: {}", self.score), 30.0);
        canvas.draw(&score_text, DrawParam::default().dest(Point2 { x: 20.0, y: 20.0 }))
    }

    fn draw_fps(&self, canvas: &mut Canvas, ctx: &Context) {
        let (arena_width, _arena_height) = ctx.gfx.drawable_size();
        let fps = ctx.time.fps();
        let fps_text = Self::create_text(format!("FPS: {:.1}", fps), 30.0);
        let dimensions = fps_text.measure(ctx).unwrap();
        canvas.draw(&fps_text, DrawParam::default().dest(Point2 { x: arena_width - dimensions.x - 20.0, y: 20.0 }))
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            GameState::Started => {
                let (_arena_width, arena_height) = ctx.gfx.drawable_size();
                self.player.apply_force(&self.gravity);
                self.player.run();
                self.player.hit_ground(arena_height);

                if ctx.keyboard.is_key_pressed(KeyCode::Space) {
                    self.player.jump(&JUMP_FORCE);
                }
                let time_elapsed = ctx.time.time_since_start().as_secs_f64();
                let increase_speed = (time_elapsed - self.time_elapsed) > INCREASE_VELOCITY_EVERY;
                for i in 0..self.obstacles.len() {
                    if let Some((obst, _obst_mesh)) = self.obstacles.get_mut(i) {
                        obst.run();
                        if obst.is_off_screen() {
                            obst.reset_location();
                            self.score += 1;
                        }
                        if increase_speed {
                            obst.increase_speed();
                        }
                        if obst.is_hitting_player(&self.player) {
                            self.state = GameState::GameOver;
                            return Ok(());
                        }
                    }
                }
                if increase_speed {
                    self.time_elapsed += INCREASE_VELOCITY_EVERY;
                    println!("Speed increasing!")
                }
            }
            GameState::GameOver => {
                if ctx.keyboard.is_key_pressed(KeyCode::Space) {
                    self.reset(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.state {
            GameState::Started => {
                // canvas.draw(&self.player_mesh, DrawParam::default().dest(self.player.get_location()));
                let player_sprite = &self.player_sprites[self.sprite_index as usize];
                canvas.draw(&self.sprite_sheet, player_sprite.draw_params(self.player.get_location()));
                self.sprite_index = (self.sprite_index+ 0.1) % self.player_sprites.len() as f32;
                for (obst, obst_mesh) in &self.obstacles {
                    canvas.draw(obst_mesh, DrawParam::default().dest(obst.get_location()));
                }
                self.draw_score(&mut canvas);
            }
            GameState::GameOver => {
                self.draw_game_over_screen(&mut canvas, ctx);
            }
        }
        self.draw_fps(&mut canvas, ctx);

        canvas.finish(ctx)
    }
}
// https://www.youtube.com/watch?v=sR1LLXegJ1E&list=PLrmY5pVcnuE9eDgLcskszIy7g0Z_hRcwk&index=12
// 23:11