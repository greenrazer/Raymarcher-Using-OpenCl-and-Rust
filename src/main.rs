extern crate ocl;
extern crate sdl2;

use std::thread::sleep;
use std::time::{Duration, Instant};

use ocl::ProQue;
use ocl::prm::{Uchar, Uchar3, Float16, Uint};
use ocl::Buffer;

use sdl2::rect::Point;
use sdl2::pixels::Color;

mod scene_objects;
use scene_objects::scene_object::SceneObject;
use scene_objects::sphere::Sphere;
use scene_objects::floor_plane::FloorPlane;

mod scene;
use scene::Scene;

const WINDOW_WIDTH: u32 = 320;
const WINDOW_HEIGHT: u32 = 180;

const SCENE_TIME_INCREMENT_BETWEEN_FRAMES: f32 = 0.01;

fn render_frame(pro_que: &ProQue, scene: &Scene) -> Result<Vec<Uchar3>, ocl::Error> {
  let pixel_buffer = pro_que.create_buffer::<Uchar3>()?;

  let (num_scene_objects, scene_object_type_buffer, scene_object_data_buffer) = scene.to_ocl_buffer(pro_que)?;

  let kernel = pro_que.kernel_builder("rayCast")
  .arg(&pixel_buffer)
  .arg(&scene_object_type_buffer)
  .arg(&scene_object_data_buffer)
  .arg(num_scene_objects)
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

  let mut scene_objs: Vec<Box<dyn SceneObject>> = Vec::new();
  scene_objs.push(Box::new(Sphere::new((-6.,3.,10.), 3.)));
  scene_objs.push(Box::new(Sphere::new((6.,3.,10.), 3.)));
  scene_objs.push(Box::new(FloorPlane::new(0.)));

  let scene = Scene::new(scene_objs);

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
    let pixels = render_frame(&pro_que, &scene).expect("error rendering frame.");

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