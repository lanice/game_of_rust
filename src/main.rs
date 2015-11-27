
#[macro_use]
extern crate glium;
// extern crate image;

use std::time::Duration;
use std::thread;

use std::fs::File;
use std::io::Read;

use glium::{DisplayBuild, Surface};


const GRID_SIZE: usize = 51;
const ITERATION_SPEED: u64 = 100;
const INITIAL_DELAY: u64 = 2000;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}


fn geometry() -> (Vec<Vertex>) {

    let vertex1 = Vertex { position: [-0.75, -0.75], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.75, -0.75], tex_coords: [1.0, 0.0] };
    let vertex3 = Vertex { position: [ 0.75,  0.75], tex_coords: [1.0, 1.0] };

    let vertex4 = Vertex { position: [ 0.75,  0.75], tex_coords: [1.0, 1.0] };
    let vertex5 = Vertex { position: [-0.75,  0.75], tex_coords: [0.0, 1.0] };
    let vertex6 = Vertex { position: [-0.75, -0.75], tex_coords: [0.0, 0.0] };

    vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}

// fn program(display: &glium::glutin::Window) -> (glium::Program) {

//     let mut vertex_shader_src = String::new();
//     let mut fragment_shader_src = String::new();
//     File::open("data/vertex_shader.glsl").unwrap().read_to_string(&mut vertex_shader_src).unwrap();
//     File::open("data/fragment_shader.glsl").unwrap().read_to_string(&mut fragment_shader_src).unwrap();

//     glium::Program::from_source(&display, &mut vertex_shader_src, &mut fragment_shader_src, None).unwrap()
// }

// fn is_cell_alive(board: & Vec<Vec<bool>>, row: usize, column: usize) -> Result<bool, &'static str> {
fn is_cell_alive(board: & Vec<Vec<bool>>, row: usize, column: usize) -> bool {
    match board.get(row) {
        None => false,
        Some(vec) => match vec.get(column) {
            None | Some(&false) => false,
            Some(&true) => true,
        },
    }
}

fn num_neighbours(board: & Vec<Vec<bool>>, row: usize, column: usize) -> u8 {
    let mut count: u8 = 0;

    if row > 0 && column > 0 {
        if is_cell_alive(board, row-1, column-1) { count = count + 1; }
    }
    if row > 0 {
        if is_cell_alive(board, row-1, column  ) { count = count + 1; }
        if is_cell_alive(board, row-1, column+1) { count = count + 1; }
    }
    if column > 0 {
        if is_cell_alive(board, row  , column-1) { count = count + 1; }
        if is_cell_alive(board, row+1, column-1) { count = count + 1; }
    }
    if is_cell_alive(board, row  , column+1) { count = count + 1; }
    if is_cell_alive(board, row+1, column  ) { count = count + 1; }
    if is_cell_alive(board, row+1, column+1) { count = count + 1; }

    count
}

fn game_board_start_config() -> Vec<Vec<bool>> {
    let mut game_board = vec![ vec![ false ; GRID_SIZE ] ; GRID_SIZE ];

    let index = GRID_SIZE / 2;

    *game_board.get_mut(index-2).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index+1).unwrap().get_mut(index).unwrap() = true;

    *game_board.get_mut(index-5-2).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index-5-1).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index-5-1).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index-5).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index-5).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index-5).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index-5+1).unwrap().get_mut(index).unwrap() = true;

    *game_board.get_mut(index+5-2).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index+5-1).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index+5-1).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index+5).unwrap().get_mut(index-1).unwrap() = true;
    *game_board.get_mut(index+5).unwrap().get_mut(index).unwrap() = true;
    *game_board.get_mut(index+5).unwrap().get_mut(index+1).unwrap() = true;
    *game_board.get_mut(index+5+1).unwrap().get_mut(index).unwrap() = true;

    *game_board.get_mut(index-2).unwrap().get_mut(index-3).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index-3-1).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index-3+1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index-3-1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index-3).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index-3+1).unwrap() = true;
    *game_board.get_mut(index+1).unwrap().get_mut(index-3).unwrap() = true;

    *game_board.get_mut(index-2).unwrap().get_mut(index+3).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index+3-1).unwrap() = true;
    *game_board.get_mut(index-1).unwrap().get_mut(index+3+1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index+3-1).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index+3).unwrap() = true;
    *game_board.get_mut(index).unwrap().get_mut(index+3+1).unwrap() = true;
    *game_board.get_mut(index+1).unwrap().get_mut(index+3).unwrap() = true;

    game_board
}

fn game_board_iterate(current: & Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();

    for row_i in 0..GRID_SIZE-1 {
        next.push(Vec::new());

        for column_i in 0..GRID_SIZE-1 {
            let neighbours: u8 = num_neighbours(&current, row_i, column_i);
            let current_value: bool = *current.get(row_i).unwrap().get(column_i).unwrap();

            next.last_mut().unwrap().push(
                match current_value {
                    true => match neighbours {
                        2|3 => true,
                        _ => false,
                    },
                    false => match neighbours {
                        3 => true,
                        _ => false,
                    },
                }
            );
        }
    }

    next
}

fn texture_vec_from_board(board: & Vec<Vec<bool>>) -> Vec<Vec<(f32, f32, f32, f32)>> {
    let mut tex_vec: Vec<Vec<(f32, f32, f32, f32)>> = Vec::<Vec<(f32, f32, f32, f32)>>::new();

    for row in board {
        tex_vec.push(Vec::new());

        for &value in row {
            tex_vec.last_mut().unwrap().push(
                match value {
                    true => (0.0, 0.0, 0.0, 0.0),
                    false => (1.0, 1.0, 1.0, 1.0),
                }
            );
        }
    }

    tex_vec
}

fn main() {

    let (monitor_width, monitor_height) = glium::glutin::get_primary_monitor().get_dimensions();
    let display = glium::glutin::WindowBuilder::new().with_dimensions(monitor_width, monitor_height).build_glium().unwrap();


    // Geometry
    implement_vertex!(Vertex, position, tex_coords);
    let vertex_buffer = glium::VertexBuffer::new(&display, &geometry()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    // Program
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    File::open("data/vertex_shader.glsl").unwrap().read_to_string(&mut vertex_shader_src).unwrap();
    File::open("data/fragment_shader.glsl").unwrap().read_to_string(&mut fragment_shader_src).unwrap();
    let program = glium::Program::from_source(&display, &mut vertex_shader_src, &mut fragment_shader_src, None).unwrap();


    let mut game_board = game_board_start_config();
    
    // let tex = vec![(1.0, 0.0, 0.0, 1.0)];
    // let image = glium::texture::RawImage2d::from_raw_rgba(tex, (1, 1));
    // let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    // let texture = glium::texture::Texture2d::with_format(&display, tex, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap).unwrap();


    let mut delay = true;

    loop {

        let tex = texture_vec_from_board(&game_board);
        let texture = glium::texture::Texture2d::new(&display, tex).unwrap();
        let sampler = glium::uniforms::Sampler::new(&texture)
                        .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let (width, height) = display.get_window().unwrap().get_inner_size_points().unwrap();
        let uniforms = uniform! {
            width: width as f32,
            height: height as f32,
            tex: sampler
        };

        let mut target = display.draw();
        target.clear_color(0.1, 0.2, 0.3, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        game_board = game_board_iterate(&game_board);


        match delay {
            true => {
                delay = false;
                thread::sleep(Duration::from_millis(INITIAL_DELAY));
            },
            false => thread::sleep(Duration::from_millis(ITERATION_SPEED))
        }

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

    }

}
