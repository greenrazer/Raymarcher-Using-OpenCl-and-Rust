extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar8, Float16};

const CAPSULE_KEY: u8 = 2;

pub struct Capsule {
  position1: (f32, f32, f32),
  position2: (f32, f32, f32),
  radius: f32,
  color: (u8, u8, u8),
  reflectivity: f32
}

impl Capsule {
  pub fn new(position1: (f32, f32, f32), position2: (f32, f32, f32), radius: f32, color: (u8, u8, u8), reflectivity: f32) -> Self {
    Capsule {position1: position1, position2: position2, radius: radius, color:color, reflectivity: reflectivity}
  }
}

impl SceneObject for Capsule {
  fn get_float_data(&self) -> Float16 {
    Float16::new(self.position1.0,self.position1.1,self.position1.2,self.position2.0,self.position2.1,self.position2.2,self.radius,0.,0.,0.,0.,0.,0.,0.,0.,self.reflectivity)
  }
  fn get_integer_data(&self) -> Uchar8 {
    Uchar8::new(CAPSULE_KEY, self.color.0, self.color.1, self.color.2, 0, 0, 0, 0)
  }
}