extern crate ocl;
extern crate fast_inv_sqrt;

use ocl::prm::{Float8};

use fast_inv_sqrt::InvSqrt32;

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
  fn add_vectors(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
  }
  fn sub_vectors(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
  }
  fn cross_vectors(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    (a.1*b.2 - a.2*b.1, a.2*b.0 - a.0*b.2, a.0*b.1 - a.1*b.0)
  }
  fn dot_vectors(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    a.0*b.0 + a.1*b.1 + a.2*b.2
  }
  //proj a onto b. b must be normalized.
  fn proj_vectors(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    Camera::scale_vector(b, Camera::dot_vectors(a,b))
  }
  //proj a onto plane defined by normal n. n must be normalized.
  fn proj_onto_plane(a: (f32, f32, f32), n: (f32, f32, f32)) -> (f32, f32, f32) {
    Camera::sub_vectors(a, Camera::proj_vectors(a, n))
  }
  fn fast_inv_length(a:(f32,f32,f32)) -> f32 {
    Camera::dot_vectors(a, a).inv_sqrt32()
  }
  fn fast_normalize(a:(f32,f32,f32)) -> (f32, f32, f32) {
    let len = Camera::fast_inv_length(a);
    (a.0*len, a.1*len, a.2*len)
  }
  fn scale_vector(a: (f32, f32, f32), scale: f32) -> (f32, f32, f32) {
    (a.0*scale, a.1*scale, a.2*scale)
  }
  pub fn move_forward(&mut self, dist: f32) {
    self.position = Camera::add_vectors(self.position, Camera::scale_vector(self.look_dir, dist));
  }
  pub fn move_right(&mut self, dist: f32) {
    self.position = Camera::add_vectors(self.position, Camera::scale_vector(self.right_dir, dist));
  }
  pub fn move_up(&mut self, dist: f32) {
    self.position = Camera::add_vectors(self.position, Camera::scale_vector(self.up_dir, dist));
  }
  pub fn set_position(&mut self, pos: (f32, f32, f32)) {
    self.position = pos;
  }
  pub fn rotate_x(&mut self, rad: f32) {
    self.rotation.0 += rad;
    self.calculate_rotation_info();
  }
  pub fn rotate_y(&mut self, rad: f32) {
    self.rotation.1 += rad;
    self.calculate_rotation_info();
  }
  pub fn rotate_z(&mut self, rad: f32) {
    self.rotation.2 += rad;
    self.calculate_rotation_info();
  }
  pub fn set_x(&mut self, rad: f32) {
    self.rotation.0 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_y(&mut self, rad: f32) {
    self.rotation.1 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_z(&mut self, rad: f32) {
    self.rotation.2 = rad;
    self.calculate_rotation_info();
  }
  pub fn set_rotation(&mut self, rads: (f32, f32, f32)) {
    self.rotation = rads;
    self.calculate_rotation_info();
  }
  pub fn look_at(&mut self, point: (f32, f32, f32)){
    let to_point = Camera::fast_normalize(Camera::sub_vectors(point, self.position));

    //for yaw find the rotation between the positive z axis and the point
    let to_pointxz = Camera::fast_normalize((to_point.0, 0., to_point.2));
    let mut angley = to_pointxz.2.acos();
    let crossy = Camera::cross_vectors((0.,0.,1.), to_pointxz);
    angley = if crossy.1 < 0. {-angley} else {angley};
    self.set_y(angley);
    
    // for pitch find the rotation between the look vector projected on the xz axis
    // and the point projected onto the yz axis.
    let to_point_look_up = Camera::fast_normalize(Camera::proj_onto_plane(to_point, self.right_dir));
    let lookxz = Camera::fast_normalize((self.look_dir.0, 0., self.look_dir.2));
    let mut angle_look = Camera::dot_vectors(lookxz, to_point_look_up).acos();
    let crossx = Camera::cross_vectors(lookxz, to_point_look_up);
    angle_look = if Camera::dot_vectors(self.right_dir, crossx) < 0. {-angle_look} else {angle_look};

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