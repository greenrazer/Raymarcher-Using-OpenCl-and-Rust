extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Float16};

const SPHERE_KEY: u8 = 0;

pub struct Sphere {
  position: (f32, f32, f32),
  radius: f32
}

impl Sphere {
  pub fn new(position: (f32, f32, f32), radius: f32) -> Self {
    Sphere {position: position, radius: radius}
  }
}

impl SceneObject for Sphere {
  fn get_type(&self) -> Uchar {
    Uchar::new(SPHERE_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.position.0,self.position.1,self.position.2,self.radius,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.)
  }
}