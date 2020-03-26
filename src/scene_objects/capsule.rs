extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Float16};

const CAPSULE_KEY: u8 = 2;

pub struct Capsule {
  position1: (f32, f32, f32),
  position2: (f32, f32, f32),
  radius: f32
}

impl Capsule {
  pub fn new(position1: (f32, f32, f32), position2: (f32, f32, f32), radius: f32) -> Self {
    Capsule {position1: position1, position2: position2, radius: radius}
  }
}

impl SceneObject for Capsule {
  fn get_type(&self) -> Uchar {
    Uchar::new(CAPSULE_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.position1.0,self.position1.1,self.position1.2,self.position2.0,self.position2.1,self.position2.2,self.radius,0.,0.,0.,0.,0.,0.,0.,0.,0.)
  }
}