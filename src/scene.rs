extern crate ocl;
use ocl::Buffer;
use ocl::ProQue;

use crate::scene_objects::scene_object::SceneObject;
use ocl::prm::{Uchar8, Float16};
use ocl::flags::MemFlags;


pub struct Scene {
  scene_objects: Vec<Box<dyn SceneObject>>
}

impl Scene {
  pub fn new() -> Self {
    Scene {scene_objects: Vec::new()}
  }

  pub fn push(&mut self, obj: Box<dyn SceneObject>) {
    self.scene_objects.push(obj);
  }

  fn to_ocl_format(&self) -> (Vec<Uchar8>, Vec<Float16>) {
    let mut objects_integer_data = Vec::<Uchar8>::with_capacity(self.scene_objects.len());
    let mut objects_float_data = Vec::<Float16>::with_capacity(self.scene_objects.len());
    for object in &self.scene_objects {
      objects_integer_data.push(object.get_integer_data());
      objects_float_data.push(object.get_float_data());
    }
    (objects_integer_data, objects_float_data)
  }

  pub fn to_ocl_buffer(&self, pro_que: &ProQue) -> Result<(u32, Buffer<Uchar8>, Buffer<Float16>), ocl::Error>{
    let (objects_integer_data, objects_float_data) = self.to_ocl_format();

    let scene_object_integer_buffer = pro_que.buffer_builder::<Uchar8>()
      .len(self.scene_objects.len())
      .flags(MemFlags::READ_ONLY)
      .build()?;
    let scene_object_float_buffer = pro_que.buffer_builder::<Float16>()
      .len(self.scene_objects.len())
      .flags(MemFlags::READ_ONLY)
      .build()?;

    scene_object_integer_buffer.write(objects_integer_data.as_slice()).enq()?;
    scene_object_float_buffer.write(objects_float_data.as_slice()).enq()?;

    Ok((self.scene_objects.len() as u32, 
        scene_object_integer_buffer, 
        scene_object_float_buffer))
  }
}