extern crate ocl;
extern crate sdl2;

use std::thread::sleep;
use std::time::{Duration, Instant};

use ocl::ProQue;
use ocl::prm::{Uchar, Uchar3, Float3, Float16, Uint};
use ocl::Buffer;

use sdl2::rect::Point;
use sdl2::pixels::Color;

mod scene_objects;
use scene_objects::scene_object::SceneObject;
use scene_objects::sphere::Sphere;
use scene_objects::floor_plane::FloorPlane;
use scene_objects::capsule::Capsule;
use scene_objects::cylinder::Cylinder;
use scene_objects::boxx::Boxx;

mod scene;
use scene::Scene;

mod camera;
use camera::Camera;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

const SCENE_TIME_INCREMENT_BETWEEN_FRAMES: f32 = 0.01;

fn render_frame(pro_que: &ProQue, camera: &Camera, scene: &Scene) -> Result<Vec<Uchar3>, ocl::Error> {
  let pixel_buffer = pro_que.create_buffer::<Uchar3>()?;

  let (num_scene_objects, scene_object_type_buffer, scene_object_data_buffer) = scene.to_ocl_buffer(pro_que)?;

  let point_light_pos = Float3::new(0.,10.,5.);

  let kernel = pro_que.kernel_builder("rayCast")
  .arg(&pixel_buffer)
  .arg(&scene_object_type_buffer)
  .arg(&scene_object_data_buffer)
  .arg(num_scene_objects)
  .arg(camera.get_data())
  .arg(point_light_pos)
  .arg(WINDOW_WIDTH)
  .arg(WINDOW_HEIGHT)
  .build()?;

  unsafe { 
    kernel.enq()?;
  }

  let mut pixels = vec![Uchar3::zero(); pixel_buffer.len()];
  pixel_buffer.read(&mut pixels).enq()?;

  Ok(pixels)
}

fn main(){

  // Setup Window and Canvas
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let window = video_subsystem
    .window("Ray Tracing Demo", WINDOW_WIDTH, WINDOW_HEIGHT)
    .build()
    .unwrap();
  
  let mut canvas = window
    .into_canvas()
    // .present_vsync()
    .build()
    .unwrap();

  let src = include_str!("opencl/kernel.cl");

  let pro_que = ProQue::builder()
      .src(src)
      .dims(WINDOW_WIDTH*WINDOW_HEIGHT)
      .build()
      .expect("Could not build ProQue.");

  let mut time: f32 = 0.;
  let mut frames: u64 = 0;
  
  let mut scene = Scene::new();
  scene.push(Box::new(Sphere::new((-6.,3.,10.), 3.)));
  // scene.push(Box::new(Sphere::new((6.,3.,10.), 3.)));
  scene.push(Box::new(FloorPlane::new(0.)));
  scene.push(Box::new(Capsule::new((0.,3., 10.),(0.,10., 15.),3.)));
  scene.push(Box::new(Cylinder::new((-13.,1., 9.),(0.,1., 3.),0.5)));
  scene.push(Box::new(Boxx::new((4.,4.,4.),(1.,1., 1.), (std::f32::consts::FRAC_PI_8 ,std::f32::consts::FRAC_PI_8,std::f32::consts::FRAC_PI_8))));

  let fov = 9.;
  let mut camera = Camera::new((0.,8.,-2. - fov), (0.2,0.,0.), fov , 50.);

  //Draw Loop
  let mut event_pump = sdl.event_pump().unwrap();
  'main: loop {

    //Quit out of program
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. } => break 'main,
        _ => {}
      }
    }
    let start = Instant::now();

    //Render Frame
    let pixels = render_frame(&pro_que, &camera, &scene).expect("error rendering frame.");

    //Update Canvas
    for pix in 0..pixels.len() {
      let y: i32 = (pix as u32 / WINDOW_WIDTH) as i32;
      let x: i32 = (pix as u32 % WINDOW_WIDTH) as i32;
      canvas.set_draw_color(Color::RGB(pixels[pix][0], pixels[pix][1], pixels[pix][2]));
      canvas.draw_point(Point::new(x,y)).expect("Could not draw point.");
    }

    //Draw Canvas
    canvas.present();

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