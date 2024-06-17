use glium::{glutin::surface::WindowSurface, implement_vertex, index::{NoIndices, PrimitiveType}, uniform, uniforms::EmptyUniforms, Display, DrawParameters, Frame, Program, Surface, VertexBuffer};












pub fn draw_text(facade: &Display<WindowSurface>, text: &str, frame: &mut Frame, x: f32, y: f32, size: f32) {

    let size = 1.0f32;

    #[derive(Clone, Copy, Debug)]
    struct Point {
        p: [f32; 2],
    }
    implement_vertex!(Point, p);

    let mut a = vec![
        Point { p: [0.0f32, 0.0]},

        Point { p: [-0.01f32, -0.01]},
        Point { p: [-0.02f32, -0.02]},

        Point { p: [0.5f32, 0.5]},
        Point { p: [1.0f32, 1.0]},
    ];

    a.iter_mut().map(move |p| {
        p.p[0] += x;
        p.p[1] += y;
    });

    let d = DrawParameters {
        point_size: Some(3.0),
        ..Default::default()
    };

    let v = VertexBuffer::new(facade, &a).unwrap();



    let u = uniform! {matrix:
        [
            [1.0 * size, 0.0, 0.0, 0.0],
            [0.0, 1.0 * size, 0.0, 0.0],
            [0.0, 0.0, 1.0 * size, 0.0],
            [0.0, 0.0, 0.0, 1.0 * size],
        ]
    };

    let p = Program::from_source(facade, VERTEX_SHADER_TEXT, FRAGMENT_SHADER_TEXT, None).unwrap();
    frame.draw(&v, &NoIndices(PrimitiveType::Points), &p, &u, &d).unwrap();
    

}


pub const VERTEX_SHADER_TEXT: &str = r#"
    #version 140

    in vec2 p;
    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(p, 1.0, 1.0);
    }

"#;

pub const FRAGMENT_SHADER_TEXT: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }

"#;