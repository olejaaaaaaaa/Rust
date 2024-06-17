#![allow(warnings)]
#![feature(duration_millis_float)]
#![macro_use]
mod core;
mod types;
mod shaders;
//mod ui;
//mod ui;


extern crate rodio;
use iter::{IntoParallelRefIterator, ParallelIterator};
use rodio::*;


extern crate glium;
use glium::Surface;
use glium::*;
use index::{NoIndices, PrimitiveType};
use shaders::*;
use uniforms::EmptyUniforms;
use winit::{event::{KeyEvent, WindowEvent}, keyboard::{Key, KeyCode}};

use core::Point;
use std::{error::Error, fs::File, io::BufReader, thread::spawn, time::Instant};

extern crate rayon;
use rayon::*;

fn main() -> Result<(), Box<dyn Error>> {


    let s = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&s.1)?;

    sink.append(Decoder::new(BufReader::new(File::open("music.mp3").unwrap()))?);


    let main_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(640, 640)
        .with_title("ORANGE GAME")
        .build(&main_loop);


    let mut time = Instant::now();

    let shader_program_1 = Program::from_source(&display, VERTEX_SHADER_2, FRAGMENT_SHADER_1, None)?;
    let index_buffer = NoIndices(PrimitiveType::LineStrip);

    let y = -2.5;
    
    let mut b = vec![
        
    ];

    let j = 7;
    let size = j as f32 /5.0;


    for i in 0..j {
        b.push(core::Box::new(&display, (2.0*size*i as f32,  y, 2.0*size), size));      // +x
        b.push(core::Box::new(&display, (-2.0*size*(i) as f32,  y, 2.0*size), size)); //-x


        b.push(core::Box::new(&display, ( 0.0 as f32,  y, 2.0*size* (i) as f32), size));
        b.push(core::Box::new(&display, ( 0.0 as f32,  y, -2.0*size* (i) as f32), size));
        

        for n in 0..j {
            b.push(core::Box::new(&display, ( 2.0 * size * i as f32,  y, -2.0 * size* (n) as f32), size));
            b.push(core::Box::new(&display, ( 2.0 * size * i as f32,  y,  2.0 * size* (n) as f32), size));

            b.push(core::Box::new(&display, ( -2.0 * size * i as f32,  y, -2.0 * size* (n) as f32), size));
            b.push(core::Box::new(&display, ( -2.0 * size * i as f32,  y,  2.0 * size* (n) as f32), size));
        }
        

        
    }

    for i in 0..b.len() {
        b[i].set_y(&display, -0.3);
    }

    let g = spawn(move || {
        sink.sleep_until_end();
    });

    let mut angle = 1.0 * (3.14/180.0) as f32;



    let k = 0.5f32;

    let mut matrix = 
    [
        [k, 0.0, 0.0, 0.0],
        [0.0, k, 0.0, 0.0],
        [0.0, 0.0, k, 0.0],
        [0.0, 0.0, 0.0, k],
    ];

    main_loop.run(move |ev, win| {

        


        

        match ev {

            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },

            winit::event::Event::WindowEvent { window_id, event } => {
                match event {

                    winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {

                        if event.state.is_pressed()  {
                            
                            if event.physical_key == KeyCode::KeyW {
                                
                                for i in 0..b.len() {
                                    b[i].set_y(&display, -0.05);
                                }

                            }

                            if event.physical_key == KeyCode::KeyS {
                                for i in 0..b.len() {
                                    b[i].set_y(&display, 0.05);
                                }
                            }

                            if event.physical_key == KeyCode::KeyA {

                                for i in 0..b.len() {
                                    b[i].set_x(&display, 0.05);
                                }
                                
                            }

                            if event.physical_key == KeyCode::KeyD {
                                for i in 0..b.len() {
                                    b[i].set_x(&display,  -0.05);
                                }
                            }
                        } 
                        
                    },

                    WindowEvent::CloseRequested => {
                        win.exit();
                    }

                    WindowEvent::RedrawRequested => {

                        let t = time.elapsed().as_secs_f32();


                        

                        let view = view_matrix(&[0.05, -1.0, 0.0], &[-2.0, 1.0, 0.0], &[0.0, 1.0, 0.0]);


                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 0.0, 1.0);



                        let mut perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;
    
                            let fov: f32 = (3.141592 / (2.0));
                            let zfar = 2.0 * 2048.0;
                            let znear = 0.5;
    
                            let f = 1.0 / (fov / 2.0).tan();
    
                            [
                                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                                [         0.0        ,     f ,              0.0              ,   0.0],
                                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                                [         0.0      ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                            ]
                        };




                        //ui::draw_text(&display, "Opa", &mut target, 1.0, 1.0, 24.0);
                        //b[0].set_x(&display, t.cos());
                        for i in 0..b.len() {
                            target.draw(&b[i].buf, index_buffer, &shader_program_1, (&uniform!{matrix: matrix, view: view, camera: perspective}), &Default::default());
                        }


                        
                        target.finish();

                    }



                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }, 

                    _ => ()
                }

   
            }

            _ => ()
        }
    });    

    Ok(())
}