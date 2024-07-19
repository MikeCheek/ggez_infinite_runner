use ggez::mint::{Point2, Vector2};

pub trait AddVector {
    fn add_vector(&mut self, other: &Vector2<f32>);
}

// Implement the trait for Point2
impl AddVector for Point2<f32> {
    fn add_vector(&mut self, other: &Vector2<f32>) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Implement the trait for Vector2
impl AddVector for Vector2<f32> {
    fn add_vector(&mut self, other: &Vector2<f32>) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub trait AddScalar {
    fn add(&mut self, other: f32);
}

impl AddScalar for Vector2<f32>{
    fn add(&mut self, other: f32) {
        self.x+=other;
        self.y+=other;
    }
}

impl AddScalar for Point2<f32>{
    fn add(&mut self, other: f32) {
        self.x+=other;
        self.y+=other;
    }
}

pub trait MultiplyScalar {
    fn multiply(&mut self, other: f32);
}

impl MultiplyScalar for Vector2<f32>{
    fn multiply(&mut self, other: f32) {
        self.x*=other;
        self.y*=other;
    }
}

impl MultiplyScalar for Point2<f32>{
    fn multiply(&mut self, other: f32) {
        self.x*=other;
        self.y*=other;
    }
}