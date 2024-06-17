#![allow(warnings)]
#![feature(duration_millis_float)]
#![macro_use]
extern crate glium;
use glium::Surface;
use glium::*;

use std::f64::consts::PI;
use std::time::Instant;





fn main() {


    
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_inner_size(640, 640)
        .with_title("Avokado")
        .build(&event_loop);
    


    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f64; 2],
    }
    implement_vertex!(Vertex, position);

    let mut shape = vec![
        Vertex { position: [ 0.0,  0.0] },    
    ];

    for i in 0..3*3 - 1 {
        shape.push(Vertex { position: [ 0.0,  0.0] });
    }


    #[derive(Copy, Clone, Debug)]
    struct Color {
        color: [f64; 3],
    }
    implement_vertex!(Color, color);

    let mut color = vec![Color{ color: [0.0, 0.0, 0.0]}];

    for i in 0..shape.len()-1 {
        color.push(Color{ color: [0.0, 0.0, 0.0]});
    }



    let mut shape2 = shape.clone();


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        
        uniform float _Time;
        
        out float Time;
        out vec3 Color;

        void main() {
            gl_Position = vec4(position, sin(Time), 1.0);
            Time = _Time;
            Color = color;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;
        in float Time;
        in vec3 Color;

        void main() {
            color = vec4(Color[0], Color[1], Color[2], 0.5);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut time = Instant::now();
    let mut time3 = Instant::now();

    let mut time2 = time.elapsed().as_millis_f32();

    let mut count = 1f64;

    event_loop.run(move |ev, window_target| {



        let mut target = display.draw();


        match ev {
            winit::event::Event::WindowEvent { event, .. } => 
            
            match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },


                winit::event::WindowEvent::RedrawRequested => {
                  
                        
                     
            
                        if time.elapsed().as_millis() >= 30 {


                            let R = 0.7;
                            for i in 0..shape.len() {
                                shape[i].position[0] = R * (PI/180.0 * count * i as f64).cos();
                                shape[i].position[1] = R * (PI/180.0 * count * i as f64).sin();
                            }

                            for i in 0..shape2.len() {
                                shape2[i].position[0] = 0.3 * (PI/180.0 * count * i as f64).sin();
                                shape2[i].position[1] = 0.3 * (PI/180.0 * count * i as f64).cos();
                            }

                            for i in 0..color.len() {
                                color[i] = Color{color: [1.0/i as f64, 1.5/i as f64, 0.6]};
                            }
 

                            let a = len((shape[0].position[0], shape[0].position[1]), (shape[1].position[0], shape[1].position[1]));
                            
                            count += 1.0;
                            time = Instant::now();
                        }

                        if time3.elapsed().as_millis() > 100 {
                            time2 = time3.elapsed().as_millis_f32();
                            time3 = Instant::now();
                        }    


            

                    
            
            
                    let dr = DrawParameters {
                        point_size: Some(3.0),
                        multisampling: true,
                        ..Default::default()
                    };

                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

                    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
                    let indices2 = glium::index::NoIndices(glium::index::PrimitiveType::Points);
            
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    let cb = VertexBuffer::new(&display, &color).unwrap();

                    target.draw((&vertex_buffer2, &cb), &indices, &program, &uniform!{_Time: time2 },
                        &dr);


                        

                   
                    
                },
                _ => (),
            },

            winit::event::Event::AboutToWait => {
                _window.request_redraw();
            },       

            _ => (),
        }

        target.finish().unwrap();


    })
    .unwrap();

}
