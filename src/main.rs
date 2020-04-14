extern crate ocl;
extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseMode, KeyRepeat, MouseButton};

use std::time::Instant;
#[allow(unused_imports)]
use std::f32::consts::{FRAC_PI_8, FRAC_PI_4, FRAC_PI_2, PI};
use std::collections::HashSet;

use ocl::ProQue;
use ocl::prm::{Uint, Float3};

mod scene_objects;
use scene_objects::sphere::Sphere;
use scene_objects::floor_plane::FloorPlane;
use scene_objects::capsule::Capsule;
use scene_objects::cylinder::Cylinder;
use scene_objects::boxx::Boxx;

mod scene;
use scene::Scene;

mod camera;
use camera::Camera;

mod vector3;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

const SCENE_TIME_INCREMENT_BETWEEN_FRAMES: f32 = 0.01;

fn render_frame(pro_que: &ProQue, camera: &Camera, scene: &Scene) -> Result<Vec<Uint>, ocl::Error> {
  let pixel_buffer = pro_que.create_buffer::<Uint>()?;

  let (num_scene_objects, 
      scene_object_integer_buffer, 
      scene_object_float_buffer) = scene.to_ocl_buffer(pro_que)?;

  let point_light_pos = Float3::new(0.,20.,5.);

  let kernel = pro_que.kernel_builder("rayCast")
  .arg(&pixel_buffer)
  .arg(&scene_object_integer_buffer)
  .arg(&scene_object_float_buffer)
  .arg(num_scene_objects)
  .arg(camera.get_data())
  .arg(point_light_pos)
  .arg(WINDOW_WIDTH)
  .arg(WINDOW_HEIGHT)
  .build()?;

  unsafe { 
    kernel.enq()?;
  }

  let mut pixels = vec![Uint::zero(); pixel_buffer.len()];
  pixel_buffer.read(&mut pixels).enq()?;

  Ok(pixels)
}

fn main(){

  let mut window = Window::new(
      "Test - ESC to exit",
      WINDOW_WIDTH as usize,
      WINDOW_HEIGHT as usize,
      WindowOptions::default(),
  )
  .unwrap_or_else(|e| {
      panic!("{}", e);
  });

  let src = include_str!("opencl/kernel.cl");

  let pro_que = ProQue::builder()
      .src(src)
      .dims(WINDOW_WIDTH*WINDOW_HEIGHT)
      .build()
      .expect("Could not build ProQue.");

      
  let mut scene = Scene::new();
  scene.push(Box::new(Sphere::new((-6.,3.,10.), 3., (255, 0, 0), 1.)));
  scene.push(Box::new(FloorPlane::new(0., (255, 255, 255), 0.)));
  scene.push(Box::new(Sphere::new((0.,1.,0.), 1., (255, 255, 255), 0.1)));
  scene.push(Box::new(Sphere::new((-10.,25.,15.), 1., (255, 255, 255), 0.2)));
  scene.push(Box::new(Capsule::new((0.,3., 10.),(0.,10., 15.),3., (0, 255, 0), 0.3)));
  scene.push(Box::new(Cylinder::new((-13.,1., 9.),(0.,1., 3.),0.5, (0, 0, 255), 0.)));
  scene.push(Box::new(Boxx::new((4.,4.,4.),(1.,1., 1.), (FRAC_PI_8,FRAC_PI_8,FRAC_PI_8), (255, 0, 255), 1.)));
  scene.push(Box::new(Boxx::new((6.,3.,10.),(1.,1., 1.), (FRAC_PI_4,FRAC_PI_8,FRAC_PI_2/3.), (0, 255, 255), 0.3)));

  let mut camera = Camera::new((0.,10.,-10.), (0.,0.,0.), 100. , 20.);

  let mut time: f32 = 0.;
  let mut frames: u64 = 0;

  let mut buffer: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

  let mut last_mouse: (f32, f32) = (0.,0.);

  //limit fps to 60
  window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  //Draw Loop
  while window.is_open() && !window.is_key_down(Key::Escape) {
    let start = Instant::now();

    let mut move_forward = false;
    let mut move_left = false;
    let mut move_right = false;
    let mut move_backward = false;
    let mut move_up = false;
    let mut move_down = false;

    // Handle Keyboard Input
    window.get_keys().map(|keys| {
      for t in keys {
          match t {
            Key::W => move_forward = true,
            Key::A => move_left = true,
            Key::D => move_right = true,
            Key::S => move_backward = true,
            Key::Q => move_up = true,
            Key::E => move_down = true,
            _ => {}
          }
      }
    });

    //Handle Mouse Input
    if window.get_mouse_down(MouseButton::Left) {
      window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
        let mouse_x = mouse.0 - (WINDOW_WIDTH as f32)/2.;
        let mouse_y = (WINDOW_HEIGHT as f32)/2. - mouse.1;
        camera.yaw(0.01*(mouse_x - last_mouse.0));
        camera.pitch(0.01*(last_mouse.1 - mouse_y));
        last_mouse = (mouse_x, mouse_y);
      });
    }

    let move_speed = 0.5;
    if move_forward {
      camera.move_forward(move_speed);
    }
    if move_backward {
      camera.move_forward(-move_speed);
    }
    if move_right {
      camera.move_right(move_speed);
    }
    if move_left {
      camera.move_right(-move_speed);
    }
    if move_up {
      camera.move_up(move_speed);
    }
    if move_down {
      camera.move_up(-move_speed);
    }


    // camera.move_right(0.5);
    // if (10.*time).sin() < 0. {
    //   camera.move_forward(-0.3);
    // }
    // else {
    //   camera.move_forward(0.3);
    // }
    // camera.look_at((-6.,3.,10.));
    // if (frames % 60) as i64 - 30 < 0 {
    //   camera.look_at((-6.,3.,10.));
    // }
    // else {
    //   camera.look_at((-10.,25.,15.));
    // }
    // camera.set_yaw((10.*time).cos());
    // camera.set_pitch((10.*time).sin());

    //Render Frame
    let pixels = render_frame(&pro_que, &camera, &scene).expect("error rendering frame.");
    
    //Update Canvas
    for pix in 0..pixels.len() {
      buffer[pix] = pixels[pix][0]
      // let y: i32 = (pix as u32 / WINDOW_WIDTH) as i32;
      // let x: i32 = (pix as u32 % WINDOW_WIDTH) as i32;
      // canvas.set_draw_color(Color::RGB(pixels[pix][0], pixels[pix][1], pixels[pix][2]));
      // canvas.draw_point(Point::new(x,y)).expect("Could not draw point.");
    }

    window.update_with_buffer(&buffer, WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize).unwrap();

    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.draw_point(Point::new(WINDOW_WIDTH as i32/2,WINDOW_HEIGHT as i32/2)).expect("Could not draw point.");

    //Draw Canvas
    // canvas.present();

    //Print Stats
    let duration = start.elapsed().as_millis();
    let fps = 1000./(duration as f32);
    if frames % 300 == 0 {
      println!("frame {} took {}ms. fps: {}.", frames, duration, fps);
    }

    time += SCENE_TIME_INCREMENT_BETWEEN_FRAMES;
    frames+=1;
  }
}