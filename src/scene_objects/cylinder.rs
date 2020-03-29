extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Uchar3, Float16};

const CYLINDER_KEY: u8 = 3;

pub struct Cylinder {
  position1: (f32, f32, f32),
  position2: (f32, f32, f32),
  radius: f32,
  color: (u8, u8, u8)
}

impl Cylinder {
  pub fn new(position1: (f32, f32, f32), position2: (f32, f32, f32), radius: f32, color: (u8, u8, u8)) -> Self {
    Cylinder {position1: position1, position2: position2, radius: radius, color: color}
  }
}

impl SceneObject for Cylinder {
  fn get_type(&self) -> Uchar {
    Uchar::new(CYLINDER_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.position1.0,self.position1.1,self.position1.2,self.position2.0,self.position2.1,self.position2.2,self.radius,0.,0.,0.,0.,0.,0.,0.,0.,0.)
  }
  fn get_color(&self) -> Uchar3 {
    Uchar3::new(self.color.0, self.color.1, self.color.2)
  }
}