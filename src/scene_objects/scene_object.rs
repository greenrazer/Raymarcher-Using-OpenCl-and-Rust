extern crate ocl;

use ocl::prm::{Uchar, Float16};

pub trait SceneObject{
  fn get_type(&self) -> Uchar;
  fn get_data(&self) -> Float16;
}