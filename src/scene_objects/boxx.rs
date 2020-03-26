extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Float16};

const BOX_KEY: u8 = 4;

pub struct Boxx {
  position: (f32, f32, f32),
  scale: (f32, f32, f32),
  rotation: (f32, f32, f32)
}

impl Boxx {
  pub fn new(position: (f32, f32, f32), scale: (f32, f32, f32), rotation: (f32, f32, f32)) -> Self {
    Boxx {position: position, scale: scale, rotation: rotation}
  }
}

impl SceneObject for Boxx {
  fn get_type(&self) -> Uchar {
    Uchar::new(BOX_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.position.0,self.position.1,self.position.2,self.scale.0,self.scale.1,self.scale.2,self.rotation.0,self.rotation.1,self.rotation.2,0.,0.,0.,0.,0.,0.,0.)
  }
}