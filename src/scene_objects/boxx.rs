extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar8, Float16};

const BOX_KEY: u8 = 4;

pub struct Boxx {
  position: (f32, f32, f32),
  scale: (f32, f32, f32),
  rotation: (f32, f32, f32),
  color: (u8, u8, u8),
  reflectivity: f32
}

impl Boxx {
  pub fn new(position: (f32, f32, f32), scale: (f32, f32, f32), rotation: (f32, f32, f32), color: (u8, u8, u8), reflectivity: f32) -> Self {
    Boxx {position: position, scale: scale, rotation: rotation, color:color, reflectivity: reflectivity}
  }
}

impl SceneObject for Boxx {
  fn get_float_data(&self) -> Float16 {
    Float16::new(self.position.0,self.position.1,self.position.2,
      self.scale.0,self.scale.1,self.scale.2,
      self.rotation.0,self.rotation.1,self.rotation.2,
      0.,0.,0.,0.,0.,0.,self.reflectivity)
  }
  fn get_integer_data(&self) -> Uchar8 {
    Uchar8::new(BOX_KEY, self.color.0, self.color.1, self.color.2, 0, 0, 0, 0)
  }
}