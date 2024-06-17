#![allow(warnings)]
#![feature(duration_millis_float)]
#![macro_use]
extern crate glium;
use glium::Surface;
use glium::*;

use std::f64::consts::PI;
use std::time::Instant;


fn collision_line_vs_line(p0: ((f64, f64), (f64, f64)), p1: ((f64, f64), (f64, f64)) ) -> (f64, f64) {

    let x1 = p0.0.0;
    let y1 = p0.0.1;

    let x2 = p0.1.0;
    let y2 = p0.1.1;

    let K1 = ((y2-y1)-x1*(y2-y1))/(x2-x1);
    let B1 = y1;
    println!("{:?}*x + {:?}", K1, B1);

    let x1 = p1.0.0;
    let y1 = p1.0.1;

    let x2 = p1.1.0;
    let y2 = p1.1.1;

    let K2 = ((y2-y1)-x1*(y2-y1))/(x2-x1);
    let B2 = y1;
    println!("{:?}*x + {:?}", K2, B2);

    let X = (B2 - B1)/(K1 - K2);
    let Y = X*K2 + B2;

    println!("y: {:?}", X*K2 + B2);
    println!("y: {:?}", X*K1 + B1);


    return (X, Y);
}


fn len(point0: (f64, f64), point1: (f64, f64)) -> f64 {
    let x1 = point0.0;
    let y1 = point0.1;

    let x2 = point1.0;
    let y2 = point1.1;

    let res = (x2 - x1).powi(2) + (y2 - y1).powi(2);

    return res.sqrt()
}

fn set_angle_C(angle: i32, point: (f64, f64), line: ((f64, f64), (f64, f64))) -> ((f64, f64), (f64, f64)) {

    // сука, я сам тут хер что разберу
    let r1 = len(point, line.0);
    let tmp_point = (line.0.0, point.1);
    let p1 = len(tmp_point, point);

    let cos = p1/r1;

    let ang = cos.acos() + (angle as f64 / 57.3);

    

    let point1_x = ang.cos() * r1;
    let point1_y = ang.sin() * r1;


    let r1 = len(point, line.1);
    let tmp_point = (line.1.0, point.1);
    let p1 = len(tmp_point, point);

    let cos = p1/r1;

    let ang = cos.acos() + (angle as f64 / 57.3);

    

    let point2_x = ang.cos() * r1;
    let point2_y = ang.sin() * r1;


    return ((point1_x, point1_y), (point2_x, point2_y));
}


fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_inner_size(640, 640)
        .with_title("Glium tutorial #2")
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