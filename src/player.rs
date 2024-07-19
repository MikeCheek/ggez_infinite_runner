use ggez::{Context, GameResult};
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::mint::{Point2, Vector2};
use crate::atlas::{Atlas, Sprite};
use crate::obstacle::Obstacle;
use crate::traits::{AddVector, MultiplyScalar};

pub struct Player {
    location: Vector2<f32>,
    height: f32,
    width: f32,
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
    is_jumping: bool,
}

impl Player {
    pub fn new() -> Self {
        let acceleration = Vector2 { x: 0.0, y: 0.0 };
        let velocity = Vector2 { x: 0.0, y: 0.0 };

        Player {
            location: Vector2 { x: 0.0, y: 0.0 },
            height: 128.0,
            width: 128.0,
            is_jumping: true,
            acceleration,
            velocity,
        }
    }

    pub fn create_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let rect_bounds = Rect::new(0.0, 0.0, self.width, self.height);
        let mut binding = MeshBuilder::new();
        let mesh_data = binding
            .rectangle(DrawMode::fill(), rect_bounds, Color::WHITE)?
            .build();
        let mesh = Mesh::from_data(ctx, mesh_data);
        Ok(mesh)
    }

    pub fn create_sprites(&self, sprites: &Atlas) -> Vec<Sprite>{
        let player1 = sprites.create_sprite("RunRight01.png");
        let player2 = sprites.create_sprite("RunRight02.png");
        let player3 = sprites.create_sprite("RunRight03.png");
        let player4 = sprites.create_sprite("RunRight04.png");
        vec![player1, player2, player3, player4]
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2 { x: self.location.x, y: self.location.y }
    }

    pub fn set_location(&mut self, x:f32, y:f32) {
        self.location.x = x;
        self.location.y = y;
    }

    pub fn get_size(&self)->(f32,f32){
        (self.width, self.height)
    }

    pub fn apply_force(&mut self, force: &Vector2<f32>) {
        self.acceleration.add_vector(force);
    }

    pub fn run(&mut self) {
        self.velocity.add_vector(&self.acceleration);
        self.location.add_vector(&self.velocity);
        self.acceleration.multiply(0.0);
    }

    pub fn hit_ground(&mut self, arena_height: f32) {
        if self.location.y + self.height > arena_height {
            self.location.y = arena_height - self.height;
            self.velocity.y = 0.0;
            self.is_jumping = false;
        }
    }

    pub fn jump(&mut self, jump_force: &Vector2<f32>) {
        if !self.is_jumping {
            self.apply_force(jump_force);
            self.is_jumping = true;
        }
    }

    pub fn is_hitting_obstacle(&self, obstacle: &Obstacle) -> bool {
        let obstacle_location = obstacle.get_location();
        let (width, height) = obstacle.get_size();
        self.location.x < obstacle_location.x + width
            && self.location.x + self.width > obstacle_location.x
            && self.location.y < obstacle_location.y + height
            && self.location.y + self.height > obstacle_location.y
    }
}