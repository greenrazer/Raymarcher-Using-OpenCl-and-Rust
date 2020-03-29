extern crate ocl;

use ocl::prm::{Uchar8, Float16};

pub trait SceneObject{
  fn get_integer_data(&self) -> Uchar8;
  fn get_float_data(&self) -> Float16;
}