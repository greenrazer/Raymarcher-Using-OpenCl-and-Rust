extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar8, Float16};

const SPHERE_KEY: u8 = 0;

pub struct Sphere {
  position: (f32, f32, f32),
  radius: f32,
  color: (u8, u8, u8),
  reflectivity: f32
}

impl Sphere {
  pub fn new(position: (f32, f32, f32), radius: f32, color: (u8, u8, u8), reflectivity: f32) -> Self {
    Sphere {position: position, radius: radius, color:color, reflectivity: reflectivity}
  }
}

impl SceneObject for Sphere {
  fn get_float_data(&self) -> Float16 {
    Float16::new(self.position.0,self.position.1,self.position.2,self.radius,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,self.reflectivity)
  }
  fn get_integer_data(&self) -> Uchar8 {
    Uchar8::new(SPHERE_KEY, self.color.0, self.color.1, self.color.2, 0, 0, 0, 0)
  }
}