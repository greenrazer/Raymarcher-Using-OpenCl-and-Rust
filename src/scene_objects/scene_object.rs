extern crate ocl;

use ocl::prm::{Uchar, Uchar3, Float16};

pub trait SceneObject{
  fn get_type(&self) -> Uchar;
  fn get_data(&self) -> Float16;
  fn get_color(&self) -> Uchar3;
}