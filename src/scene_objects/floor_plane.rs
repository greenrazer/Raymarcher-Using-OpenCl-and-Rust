extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Float16};

const FLOORPLANE_KEY: u8 = 1;

pub struct FloorPlane {
  height: f32,
}

impl FloorPlane {
  pub fn new(height: f32) -> Self {
    FloorPlane {height: height}
  }
}

impl SceneObject for FloorPlane {
  fn get_type(&self) -> Uchar {
    Uchar::new(FLOORPLANE_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.height,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.)
  }
}