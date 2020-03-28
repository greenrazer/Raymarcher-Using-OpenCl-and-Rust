extern crate ocl;
extern crate fast_inv_sqrt;

use ocl::prm::{Float8};

use fast_inv_sqrt::InvSqrt32;

use crate::vector3::Vector3;

pub struct Camera {
  position: (f32, f32, f32),
  look_dir: (f32, f32, f32),
  right_dir: (f32, f32, f32),
  up_dir: (f32, f32, f32),
  rotation: (f32, f32, f32),
  rotation_sin: (f32, f32, f32),
  rotation_cos: (f32, f32, f32),
  frame_distance: f32,
  scale:f32
}

impl Camera {
  pub fn new(position: (f32, f32, f32), rotation: (f32, f32, f32), frame_distance: f32, scale: f32) -> Self {
    let mut camera = Camera {position: position, 
                        look_dir: (0.,0.,0.), 
                        right_dir: (0.,0.,0.), 
                        up_dir: (0.,0.,0.), 
                        rotation: rotation, 
                        rotation_sin: (0., 0., 0.),
                        rotation_cos: (0., 0., 0.),
                        frame_distance: frame_distance, 
                        scale: scale};
    camera.calculate_rotation_info();
    camera
  }
  fn calculate_rotation_info(&mut self) {
    self.calculate_trig_funcs();
    self.calculate_forward_vec();
    self.calculate_right_vec();
    self.calculate_up_vec();
  }
  fn calculate_trig_funcs(&mut self) {
    self.rotation_cos = (self.rotation.0.cos(),self.rotation.1.cos(),self.rotation.2.cos());
    self.rotation_sin = (self.rotation.0.sin(),self.rotation.1.sin(),self.rotation.2.sin());
  }
  fn calculate_forward_vec(&mut self) {
    let x: f32 = self.rotation_cos.2*self.rotation_sin.1*self.rotation_cos.0 + self.rotation_sin.2*self.rotation_sin.0;
    let y: f32 = self.rotation_sin.2*self.rotation_sin.1*self.rotation_cos.0 - self.rotation_cos.2*self.rotation_sin.0;
    let z: f32 = self.rotation_cos.1*self.rotation_cos.0;

    self.look_dir = (x,y,z);
  }
  fn calculate_right_vec(&mut self) {
    let x: f32 = self.rotation_cos.2*self.rotation_cos.1;
    let y: f32 = self.rotation_sin.2*self.rotation_cos.1;
    let z: f32 = -self.rotation_sin.1;

    self.right_dir = (x,y,z);
  }
  fn calculate_up_vec(&mut self) {
    let x: f32 = self.rotation_cos.2*self.rotation_sin.1*self.rotation_sin.0 - self.rotation_sin.2*self.rotation_cos.0;
    let y: f32 = self.rotation_sin.2*self.rotation_sin.1*self.rotation_sin.0 + self.rotation_cos.2*self.rotation_cos.0;
    let z: f32 = self.rotation_cos.1*self.rotation_sin.0;

    self.up_dir = (x,y,z);
  }
  pub fn move_forward(&mut self, dist: f32) {
    self.position = self.position.add(self.look_dir.scale(dist));
  }
  pub fn move_right(&mut self, dist: f32) {
    self.position = self.position.add(self.right_dir.scale(dist));
  }
  pub fn move_up(&mut self, dist: f32) {
    self.position = self.position.add(self.up_dir.scale(dist));
  }
  pub fn set_position(&mut self, pos: (f32, f32, f32)) {
    self.position = pos;
  }
  pub fn rotate_pitch(&mut self, rad: f32) {
    self.rotation.0 += rad;
    self.calculate_rotation_info();
  }
  pub fn rotate_yaw(&mut self, rad: f32) {
    self.rotation.1 += rad;
    self.calculate_rotation_info();
  }
  pub fn rotate_roll(&mut self, rad: f32) {
    self.rotation.2 += rad;
    self.calculate_rotation_info();
  }
  pub fn set_pitch(&mut self, rad: f32) {
    self.rotation.0 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_yaw(&mut self, rad: f32) {
    self.rotation.1 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_roll(&mut self, rad: f32) {
    self.rotation.2 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_rotation(&mut self, rads: (f32, f32, f32)) {
    self.rotation = rads;
    self.calculate_rotation_info();
  }
  pub fn look_at(&mut self, point: (f32, f32, f32)){
    let to_point = point.sub(self.position).fast_normalize();

    //for yaw find the rotation between the positive z axis and the point
    let to_pointxz = (to_point.0, 0., to_point.2).fast_normalize();
    let mut angley = to_pointxz.2.acos();
    let crossy = (0., 0., 1.).cross(to_pointxz);
    angley = if crossy.1 < 0. {-angley} else {angley};
    self.rotation.1 = angley;
    self.calculate_rotation_info();
    
    // for pitch find the rotation between the look vector projected on the xz axis
    // and the point projected onto the yz axis.
    let to_point_look_up = to_point.proj_onto_plane(self.right_dir).fast_normalize();
    let lookxz = (self.look_dir.0, 0., self.look_dir.2).fast_normalize();
    let mut angle_look = lookxz.dot(to_point_look_up).acos();
    let crossx = lookxz.cross(to_point_look_up);
    angle_look = if self.right_dir.dot(crossx) < 0. {-angle_look} else {angle_look};

    // the x pitch is zero when the dot product between the z axis and the point is zero.
    // it is angle_look when the dot product between the z axis and the point is one.
    // the z pitch is zero when the dot produce between the x axis and the point is zero.
    // it is angle_look when the dot product between the x axis and the point is one.
    let mut anglex = angle_look*to_point.2;
    let mut anglez = -angle_look*to_point.0;

    // when the point is facing the other way in the x direction, invert the x angle.
    if to_point.2 < 0. {
      anglex = -anglex
    }

    self.set_rotation((anglex, angley, anglez));
  }
  pub fn get_data(&self) -> Float8 {
    return Float8::new(self.position.0,self.position.1, self.position.2,self.rotation.0,self.rotation.1, self.rotation.2, self.frame_distance, self.scale)
  }
}