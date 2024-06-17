
use glium::*;

#[path ="types.rs"]
mod types;
use glutin::surface::WindowSurface;
use types::*;


#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub color: [f32; 3]
}
implement_vertex!(Color, color);

impl Color {
    fn new<T: Float>(_color: (T, T, T)) -> Self {
        Self {color: [_color.0._into(), _color.1._into(), _color.2._into()]}
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub pos: [f32; 3]
}
implement_vertex!(Point, pos);

impl Point {
    fn new<T: Float>(pos: (T, T, T)) -> Self {
        Self {
            pos: [pos.0._into(), pos.1._into(), pos.2._into()]
        }
    }
}


pub struct Box {
    pub lines: Vec<Point>,
    pub buf:   VertexBuffer<Point>
}

impl Box {
    pub fn new(facade: &Display<WindowSurface>, pos: (f32, f32, f32), size: f32) -> Self {

        let mut lines: Vec<Point> = vec![];

        let size = size * 2.0;

        lines.push(Point::new(( 0.5, 0.5, 0.5)));

        lines.push(Point::new(( -0.5, 0.5, 0.5)));

        lines.push(Point::new(( -0.5, -0.5, 0.5)));

        lines.push(Point::new((  0.5, -0.5, 0.5)));

        lines.push(Point::new(( 0.5, 0.5, 0.5)));

        lines.push(Point::new(( 0.5, 0.5, -0.5)));

        lines.push(Point::new(( -0.5, 0.5, -0.5)));

        lines.push(Point::new(( -0.5, 0.5,  0.5)));

        lines.push(Point::new(( -0.5, -0.5,  0.5)));

        lines.push(Point::new(( -0.5, -0.5,  -0.5)));

        lines.push(Point::new((  0.5, -0.5,  -0.5)));

        lines.push(Point::new((  0.5, -0.5,  0.5)));

        lines.push(Point::new((  0.5, -0.5,  -0.5)));

        lines.push(Point::new((  0.5,  0.5,  -0.5)));

        lines.push(Point::new((  -0.5,  0.5,  -0.5)));

        lines.push(Point::new((  -0.5,  -0.5,  -0.5)));



        for i in &mut lines {
            i.pos[0] = (i.pos[0] * size) + pos.0;
            i.pos[1] = (i.pos[1] * size) + pos.1;
            i.pos[2] = (i.pos[2] * size) + pos.2;
        }

        




        


        Self {
            lines: lines.clone(),
            buf: VertexBuffer::new(facade, &lines).unwrap(),
        }
    }

    pub fn set_x(&mut self, facade: &Display<WindowSurface>, x: f32) {
        
        for i in 0..self.lines.len() {
            self.lines[i].pos[0] += x; 
        }
        self.buf = VertexBuffer::new(facade, &self.lines).unwrap();        
    }

    pub fn set_y(&mut self, facade: &Display<WindowSurface>, y: f32) {
        for i in 0..self.lines.len() {
            self.lines[i].pos[1] += y; 
        }
        self.buf = VertexBuffer::new(facade, &self.lines).unwrap();        
    }

    pub fn set_z(&mut self, facade: &Display<WindowSurface>, z: f32) {
        for i in 0..self.lines.len() {
            self.lines[i].pos[2] += z; 
        }
        self.buf = VertexBuffer::new(facade, &self.lines).unwrap();        
    }
}













