
#[macro_use]
extern crate glium;

use std::time::Duration;
use std::thread;

use std::fs::File;
use std::io::Read;

use glium::{DisplayBuild, Surface};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}


fn setup_geometry() -> (Vec<Vertex>) {
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [ 0.5,  0.5] };

    let vertex4 = Vertex { position: [ 0.5,  0.5] };
    let vertex5 = Vertex { position: [-0.5,  0.5] };
    let vertex6 = Vertex { position: [-0.5, -0.5] };

    vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}

// fn setup_program(display: &glium::glutin::Window) -> (glium::Program) {
//     let mut vertex_shader_src = String::new();
//     let mut fragment_shader_src = String::new();
//     File::open("data/vertex_shader.glsl").unwrap().read_to_string(&mut vertex_shader_src).unwrap();
//     File::open("data/fragment_shader.glsl").unwrap().read_to_string(&mut fragment_shader_src).unwrap();

//     glium::Program::from_source(&display, &mut vertex_shader_src, &mut fragment_shader_src, None).unwrap()
// }

fn main() {

    let (monitor_width, monitor_height) = glium::glutin::get_primary_monitor().get_dimensions();
    let display = glium::glutin::WindowBuilder::new().with_dimensions(monitor_width, monitor_height).build_glium().unwrap();

    let shape = setup_geometry();

    implement_vertex!(Vertex, position);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    File::open("data/vertex_shader.glsl").unwrap().read_to_string(&mut vertex_shader_src).unwrap();
    File::open("data/fragment_shader.glsl").unwrap().read_to_string(&mut fragment_shader_src).unwrap();

    let program = glium::Program::from_source(&display, &mut vertex_shader_src, &mut fragment_shader_src, None).unwrap();
    

    loop {
        let (width, height) = display.get_window().unwrap().get_inner_size_points().unwrap();

        let mut target = display.draw();
        target.clear_color(0.1, 0.2, 0.3, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { width: width as f32, height: height as f32 }, &Default::default()).unwrap();
        target.finish().unwrap();

        thread::sleep(Duration::from_millis(100));

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

    }

}
