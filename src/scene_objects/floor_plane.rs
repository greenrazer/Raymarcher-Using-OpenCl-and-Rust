extern crate ocl;

use super::scene_object::SceneObject;
use ocl::prm::{Uchar, Uchar3, Float16};

const FLOORPLANE_KEY: u8 = 1;

pub struct FloorPlane {
  height: f32,
  color: (u8, u8, u8),
  reflectivity: f32
}

impl FloorPlane {
  pub fn new(height: f32, color: (u8, u8, u8), reflectivity: f32) -> Self {
    FloorPlane {height: height, color:color, reflectivity: reflectivity}
  }
}

impl SceneObject for FloorPlane {
  fn get_type(&self) -> Uchar {
    Uchar::new(FLOORPLANE_KEY)
  }
  fn get_data(&self) -> Float16 {
    Float16::new(self.height,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,self.reflectivity)
  }
  fn get_color(&self) -> Uchar3 {
    Uchar3::new(self.color.0, self.color.1, self.color.2)
  }
}