use ggez::{Context, GameResult};
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder};
use ggez::mint::{Point2, Vector2};
use crate::constants::{OBSTACLE_SPEED_INCREASE_RATE, OBSTACLE_VELOCITY};
use crate::player::Player;
use crate::traits::{ AddVector};

pub struct Obstacle {
    initial_location: Vector2<f32>,
    location: Vector2<f32>,
    height: f32,
    width: f32,
    velocity: Vector2<f32>,
}

impl Obstacle {
    pub fn new(location_x: f32, location_y: f32, size: f32, offset:f32) -> Self {
        let initial_location = Vector2 { x: location_x, y: location_y };
        let mut location = initial_location.clone();
        location.x += offset;
        let velocity = Vector2 { x: OBSTACLE_VELOCITY, y: 0.0 };

        Obstacle {
            initial_location,
            location,
            velocity,
            height: size,
            width: size,
        }
    }

    pub fn create_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let triangle_points = [
            Point2 { x: 0.0, y: 0.0 },
            Point2 { x: self.width / 2.0, y: self.height },
            Point2 { x: self.width, y: 0.0 }
        ];

        let mut binding = MeshBuilder::new();
        let mesh_data = binding
            .polyline(DrawMode::fill(), &triangle_points, Color::WHITE)?
            .build();

        let mesh = Mesh::from_data(ctx, mesh_data);
        Ok(mesh)
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

    pub fn run(&mut self) {
        // self.velocity.add_vector(&self.acceleration);
        self.location.add_vector(&self.velocity);
        // self.acceleration.multiply(0.0);
    }

    pub fn reset_location(&mut self){
        self.location = self.initial_location.clone();
    }

    pub fn is_off_screen(&self) -> bool{
        self.location.x + self.width < 0.0
    }

    pub fn increase_speed(&mut self){
        self.velocity.add_vector(&Vector2{x:OBSTACLE_SPEED_INCREASE_RATE, y:0.0})
    }

    pub fn is_hitting_player(&self, player: &Player) -> bool {
        let player_location = player.get_location();
        let (width, height) = player.get_size();

        self.location.x < player_location.x + width
            && self.location.x + self.width > player_location.x
            && self.location.y < player_location.y + height
            && self.location.y + self.height > player_location.y
    }

}