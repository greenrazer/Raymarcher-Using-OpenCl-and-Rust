extern crate ocl;
extern crate sdl2;

use std::thread::sleep;
use std::time::{Duration, Instant};

use ocl::ProQue;
use ocl::prm::{Uchar, Uchar3, Float16, Uint};
use ocl::flags::MemFlags;
use ocl::Buffer;

use sdl2::rect::Point;
use sdl2::pixels::Color;

const WINDOW_WIDTH: u32 = 320;
const WINDOW_HEIGHT: u32 = 180;

const SCENE_TIME_INCREMENT_BETWEEN_FRAMES: f32 = 0.01;

fn render_frame(pro_que: &ProQue, time: f32) -> Result<(Vec<Uchar3>, Vec<Uint>), ocl::Error> {
  let pixel_buffer = pro_que.create_buffer::<Uchar3>()?;
  let iterations_buffer = pro_que.create_buffer::<Uint>()?;

  let num_scene_objects = 3;
  let scene_object_type_buffer = pro_que.buffer_builder::<Uchar>()
                                .len(num_scene_objects)
                                .flags(MemFlags::READ_ONLY)
                                .build()?;
  let scene_object_data_buffer = pro_que.buffer_builder::<Float16>()
                                .len(num_scene_objects)
                                .flags(MemFlags::READ_ONLY)
                                .build()?;

  let obt: Vec<Uchar> = vec![
    Uchar::new(0),
    Uchar::new(0),
    Uchar::new(1)
  ];
  let data: Vec<Float16> = vec![
    Float16::new(-6.,3.,10.,3.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.),
    Float16::new(6.,3.,10.,3.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.),
    Float16::zero(),
  ];

  scene_object_type_buffer.write(obt.as_slice()).enq()?;
  scene_object_data_buffer.write(data.as_slice()).enq()?;

  let kernel = pro_que.kernel_builder("rayCast")
  .arg(&pixel_buffer)
  .arg(&iterations_buffer)
  .arg(&scene_object_type_buffer)
  .arg(&scene_object_data_buffer)
  .arg(num_scene_objects)
  .arg(WINDOW_WIDTH)
  .arg(WINDOW_HEIGHT)
  .arg(time)
  .build()?;

  unsafe { kernel.enq()?; }

  let mut pixels = vec![Uchar3::zero(); pixel_buffer.len()];
  pixel_buffer.read(&mut pixels).enq()?;
  let mut iterations = vec![Uint::zero(); iterations_buffer.len()];
  iterations_buffer.read(&mut iterations).enq()?;

  Ok((pixels, iterations))
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

    let (pixels, iterations) = render_frame(&pro_que, time).expect("error rendering frame.");

    for pix in 0..pixels.len() {
      let y: i32 = (pix as u32 / WINDOW_WIDTH) as i32;
      let x: i32 = (pix as u32 % WINDOW_WIDTH) as i32;
      canvas.set_draw_color(Color::RGB(pixels[pix][0], pixels[pix][1], pixels[pix][2]));
      canvas.draw_point(Point::new(x,y)).expect("Could not draw point.");
    }
    //Draw Canvas
    canvas.present();
    let duration = start.elapsed().as_millis();
    let fps = 1000./(duration as f32);
    if frames % 300 == 0 {
      println!("frame {} took {:?}. fps: {}.", frames, duration, fps);
    }

    time += SCENE_TIME_INCREMENT_BETWEEN_FRAMES;
    frames+=1;
    // sleep(Duration::new(0, 1_000_000_000u32 / 60))
  }
}