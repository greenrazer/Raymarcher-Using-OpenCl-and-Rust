extern crate ocl;
use ocl::prm::{Float8};

pub struct Camera {
  position: (f32, f32, f32),
  rotation: (f32, f32, f32),
  frame_distance: f32,
  scale:f32
}

impl Camera {
  pub fn new(position: (f32, f32, f32), rotation: (f32, f32, f32), frame_distance: f32, scale: f32) -> Self {
    Camera {position: position, rotation: rotation, frame_distance: frame_distance, scale: scale}
  }
  pub fn get_data(&self) -> Float8 {
    return Float8::new(self.position.0,self.position.1, self.position.2,self.rotation.0,self.rotation.1, self.rotation.2, self.frame_distance, self.scale)
  }
}