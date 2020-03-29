extern crate ocl;
use ocl::Buffer;
use ocl::ProQue;

use crate::scene_objects::scene_object::SceneObject;
use ocl::prm::{Uchar, Uchar3, Float16};
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

  fn to_ocl_format(&self) -> (Vec<Uchar>, Vec<Float16>, Vec<Uchar3>) {
    let mut object_type = Vec::<Uchar>::with_capacity(self.scene_objects.len());
    let mut object_data = Vec::<Float16>::with_capacity(self.scene_objects.len());
    let mut object_color = Vec::<Uchar3>::with_capacity(self.scene_objects.len());
    for object in &self.scene_objects {
      object_type.push(object.get_type());
      object_data.push(object.get_data());
      object_color.push(object.get_color());
    }
    (object_type, object_data, object_color)
  }

  pub fn to_ocl_buffer(&self, pro_que: &ProQue) -> Result<(u32, Buffer<Uchar>, Buffer<Float16>, Buffer<Uchar3>), ocl::Error>{
    let (objects_type, objects_data, objects_color) = self.to_ocl_format();

    let scene_object_type_buffer = pro_que.buffer_builder::<Uchar>()
      .len(self.scene_objects.len())
      .flags(MemFlags::READ_ONLY)
      .build()?;
    let scene_object_data_buffer = pro_que.buffer_builder::<Float16>()
      .len(self.scene_objects.len())
      .flags(MemFlags::READ_ONLY)
      .build()?;
    let scene_object_color_buffer = pro_que.buffer_builder::<Uchar3>()
      .len(self.scene_objects.len())
      .flags(MemFlags::READ_ONLY)
      .build()?;

    scene_object_type_buffer.write(objects_type.as_slice()).enq()?;
    scene_object_data_buffer.write(objects_data.as_slice()).enq()?;
    scene_object_color_buffer.write(objects_color.as_slice()).enq()?;

    Ok((self.scene_objects.len() as u32, 
        scene_object_type_buffer, 
        scene_object_data_buffer, 
        scene_object_color_buffer))
  }
}