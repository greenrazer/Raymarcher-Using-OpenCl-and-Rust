use fast_inv_sqrt::InvSqrt32;

pub trait Vector3 {
  fn add(self, b: (f32, f32, f32)) -> (f32, f32, f32);
  fn sub(self, b: (f32, f32, f32)) -> (f32, f32, f32);
  fn cross(self, b: (f32, f32, f32)) -> (f32, f32, f32);
  fn dot(self, b: (f32, f32, f32)) -> f32;
  fn scale(self, scale: f32) -> (f32, f32, f32);
  fn div(self, scale: f32) -> (f32, f32, f32);
  //proj a onto b. b must be normalized.
  fn proj_onto(self, dir: (f32, f32, f32)) -> (f32, f32, f32);
  //proj a onto plane defined by normal n. n must be normalized.
  fn proj_onto_plane(self, plane_norm: (f32, f32, f32)) -> (f32, f32, f32);
  fn fast_inv_length(self) -> f32;
  fn length(self) -> f32;
  fn normalize(self) -> (f32, f32, f32);
  fn fast_normalize(self) -> (f32, f32, f32);
}

impl Vector3 for (f32, f32, f32) {
  fn add(self, b: (f32, f32, f32)) -> (f32, f32, f32) {
    (self.0 + b.0, self.1 + b.1, self.2 + b.2)
  }
  fn sub(self, b: (f32, f32, f32)) -> (f32, f32, f32) {
    (self.0 - b.0, self.1 - b.1, self.2 - b.2)
  }
  fn cross(self, b: (f32, f32, f32)) -> (f32, f32, f32) {
    (self.1*b.2 - self.2*b.1, self.2*b.0 - self.0*b.2, self.0*b.1 - self.1*b.0)
  }
  fn dot(self, b: (f32, f32, f32)) -> f32 {
    self.0*b.0 + self.1*b.1 + self.2*b.2
  }
  fn scale(self, scale: f32) -> (f32, f32, f32) {
    (self.0*scale, self.1*scale, self.2*scale)
  }
  fn div(self, scale: f32) -> (f32, f32, f32) {
    (self.0/scale, self.1/scale, self.2/scale)
  }
  //proj a onto b. b must be normalized.
  fn proj_onto(self, dir: (f32, f32, f32)) -> (f32, f32, f32) {
    dir.scale(self.dot(dir))
  }
  //proj a onto plane defined by normal n. n must be normalized.
  fn proj_onto_plane(self, plane_norm: (f32, f32, f32)) -> (f32, f32, f32) {
    self.sub(self.proj_onto(plane_norm))
  }
  fn fast_inv_length(self) -> f32 {
    self.dot(self).inv_sqrt32()
  }
  fn length(self) -> f32 {
    self.dot(self).sqrt()
  }
  fn normalize(self) -> (f32, f32, f32) {
    let len = self.length();
    self.div(len)
  }
  fn fast_normalize(self) -> (f32, f32, f32) {
    let inv_len = self.fast_inv_length();
    self.scale(inv_len)
  }
}