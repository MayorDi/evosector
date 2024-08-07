use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::Mutex;

use evosector::camera::Camera;
use evosector::cell::Cell;
use evosector::constants::SIZE_GRID;
use evosector::genome::gene::Gene;
use evosector::grid::Grid;
use evosector::mouse::Mouse;
use evosector::opengl::prelude::*;
use evosector::traits::Behavior;
use evosector::traits::Render;
use glfw::Action;
use glfw::Context;
use glfw::Key;
use glfw::PWindow;
use glfw::WindowEvent;
use nalgebra::Vector2;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (window, events) = glfw
        .create_window(1200, 600, "evosector", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let window = Arc::new(Mutex::new(window));
    {
        let window = window.clone();
        let mut window = window.lock().unwrap();
        gl::load_with(|s| window.get_proc_address(s));
        window.set_framebuffer_size_callback(|_, w, h| unsafe {
            gl::Viewport(0, 0, w, h);
        });

        window.make_current();
        window.set_all_polling(true);
    }

    let programs = Arc::new(Mutex::new(HashMap::new()));
    let vao_vbo = Arc::new(Mutex::new(HashMap::new()));
    let textures_ids = Arc::new(Mutex::new(HashMap::new()));

    let camera = Arc::new(Mutex::new(Camera::new()));
    let mouse = Arc::new(Mutex::new(Mouse::new()));
    let mut time: u32 = 0;
    let mut grid = Grid::generate(0);

    let cells = Arc::new(Mutex::new(vec![Cell::new(Vector2::new(0.5, 0.5))]));
    let grid = Arc::new(Mutex::new(grid));
    {
        let cells = cells.clone();
        let cells = cells.lock().unwrap();
        let grid = grid.clone();
        let mut grid = grid.lock().unwrap();
        grid.sectors[cells[0].idx_sector].count_of_cells += 1.0;
    }

    let (vao_grid, texture_grid) = generate_tools_render_grid();
    let mut vao_cells = 0;
    let mut vbo_cells = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao_cells);
        gl::GenBuffers(1, &mut vbo_cells);

        gl::BindVertexArray(vao_cells);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_cells);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (6 * 3 * std::mem::size_of::<f32>()) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    //SHADERS PROGRAMS:
    //SHADER PROGRAM FOR GRID:
    let shader_v_grid = std::fs::read("./assets/shaders/grid.vert").unwrap();
    let shader_f_grid = std::fs::read("./assets/shaders/grid.frag").unwrap();

    let vs_grid = Shader::new(gl::VERTEX_SHADER, shader_v_grid);
    let fs_grid = Shader::new(gl::FRAGMENT_SHADER, shader_f_grid);
    let mut program_grid = Program::new();
    program_grid.push_shader(vs_grid);
    program_grid.push_shader(fs_grid);
    program_grid.build().unwrap();

    // SHADER PROGRAM FOR CELLS:
    let shader_v_cells = std::fs::read("./assets/shaders/cell.vert").unwrap();
    let shader_f_cells = std::fs::read("./assets/shaders/cell.frag").unwrap();

    let vs_cells = Shader::new(gl::VERTEX_SHADER, shader_v_cells);
    let fs_cells = Shader::new(gl::FRAGMENT_SHADER, shader_f_cells);
    let mut program_cells = Program::new();
    program_cells.push_shader(vs_cells);
    program_cells.push_shader(fs_cells);
    program_cells.build().unwrap();
    // END SHADERS PROGRAMS;

    // FOR RENDER:
    {
        let mut vao_vbo = vao_vbo.lock().unwrap();
        let mut programs = programs.lock().unwrap();
        let mut textures_ids = textures_ids.lock().unwrap();
        vao_vbo.insert("vao_grid", vao_grid);
        vao_vbo.insert("vao_cells", vao_cells);
        vao_vbo.insert("vbo_cells", vbo_cells);
        programs.insert("program_grid", program_grid);
        programs.insert("program_cells", program_cells);
        textures_ids.insert("texture_grid", texture_grid);
    }
    // ===

    while !window_status(window.clone()) {
        {
            let arc_cells = cells.clone();
            let cells = arc_cells.lock().unwrap();
            let window = window.clone();
            window
                .lock()
                .unwrap()
                .set_title(format!("evosector | count_cells: {}", cells.len()).as_str());
        }

        let update = {
            let grid = grid.clone();
            let cells = cells.clone();
            std::thread::spawn(move || {
                let mut grid = grid.lock().unwrap();
                let mut count_cells = { cells.lock().unwrap().len() };
                let mut i = 0;
                while i < count_cells {
                    let mut cells = cells.lock().unwrap();
                    let glob_gene = cells[i].update(&mut grid);
                    if let Some(gene) = glob_gene {
                        match gene {
                            Gene::Reproduction => {
                                let reprod = cells[i].reproduction();
                                if let Some(cell) = reprod {
                                    grid.sectors[cell.idx_sector].count_of_cells += 1.0;
                                    cells.push(cell);
                                }
                            }
                            _ => {}
                        }
                    }

                    if !cells[i].is_alive {
                        cells.remove(i);
                        count_cells -= 1;
                    } else {
                        i += 1;
                    }
                }
            })
        };

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            let camera = camera.clone();
            let mut camera = camera.lock().unwrap();

            let mouse = mouse.clone();
            let mut mouse = mouse.lock().unwrap();

            let window = window.clone();
            let mut window = window.lock().unwrap();

            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    mouse.button = button;

                    match action {
                        Action::Press => mouse.pressed = true,
                        _ => mouse.pressed = false,
                    }
                }

                glfw::WindowEvent::Scroll(_, y) => {
                    if (camera.scale + y as f32) > 0.0 {
                        camera.scale += y as f32;
                    }
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    mouse.old_position = mouse.position;
                    mouse.position = Vector2::new(x as f32, y as f32);

                    if mouse.pressed {
                        match mouse.button {
                            glfw::MouseButton::Button1 => camera.position += mouse.delta(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        unsafe {
            let window = window.clone();
            let camera = camera.clone();
            let time = Arc::new(time);
            let programs = programs.clone();
            let textures_ids = textures_ids.clone();
            let vao_vbo = vao_vbo.clone();
            let cells = cells.clone();

            render(window, camera, time, cells, programs, vao_vbo, textures_ids);
        }

        time += 1;

        update.join().unwrap();
        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000 / 60));
    }
}

fn window_status(window: Arc<Mutex<PWindow>>) -> bool {
    let window = window.lock().unwrap();
    window.should_close()
}

unsafe fn render(
    window: Arc<Mutex<PWindow>>,
    camera: Arc<Mutex<Camera>>,
    time: Arc<u32>,
    cells: Arc<Mutex<Vec<Cell>>>,
    programs: Arc<Mutex<HashMap<&str, Program<Shader>>>>,
    vao_vbo: Arc<Mutex<HashMap<&str, u32>>>,
    textures_ids: Arc<Mutex<HashMap<&str, u32>>>,
) {
    let mut window = window.lock().unwrap();
    let texture_grid = textures_ids.lock().unwrap()["texture_grid"];
    let programs = &programs.lock().unwrap();
    let program_grid = &programs["program_grid"];
    let program_cells = &programs["program_cells"];
    let camera = camera.lock().unwrap();
    let (vao_grid, vao_cells, vbo_cells) = {
        let vao_vbo = vao_vbo.lock().unwrap();
        (
            vao_vbo["vao_grid"],
            vao_vbo["vao_cells"],
            vao_vbo["vbo_cells"],
        )
    };

    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    // REBDER GRID
    gl::BindTexture(gl::TEXTURE_2D, texture_grid);

    gl::UseProgram(program_grid.id());
    gl::Uniform3f(
        get_location(program_grid, "resolution"),
        window.get_size().0 as f32,
        window.get_size().1 as f32,
        0.0,
    );
    gl::Uniform2f(
        get_location(program_grid, "camera_pos"),
        camera.position.x,
        camera.position.y,
    );
    gl::Uniform1f(get_location(program_grid, "camera_scale"), camera.scale);

    gl::Uniform1ui(get_location(program_grid, "Time"), *time);
    gl::BindVertexArray(vao_grid);
    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
    gl::BindVertexArray(0);

    // RENDER CELLS
    gl::UseProgram(program_cells.id());
    gl::Uniform3f(
        get_location(program_cells, "resolution"),
        window.get_size().0 as f32,
        window.get_size().1 as f32,
        0.0,
    );
    gl::Uniform2f(
        get_location(program_cells, "camera_pos"),
        camera.position.x,
        camera.position.y,
    );
    gl::Uniform1f(get_location(program_cells, "camera_scale"), camera.scale);
    gl::BindVertexArray(vao_cells);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_cells);
    {
        let cells = cells.lock().unwrap();
        for cell in cells.iter() {
            cell.render();
        }
    }
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
    gl::UseProgram(0);

    window.swap_buffers();
}

fn generate_texture_for_grid() -> u32 {
    let mut id = 0;
    unsafe {
        gl::GenTextures(1, &mut id);

        let grid_image = image::io::Reader::open("./assets/textures/texture_grid.png")
            .unwrap()
            .decode()
            .unwrap();
        let grid_texture = grid_image.as_bytes();

        gl::BindTexture(gl::TEXTURE_2D, id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            grid_image.width() as i32,
            grid_image.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            grid_texture.as_ptr() as *const _,
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    id
}

fn generate_tools_render_grid() -> (u32, u32) {
    let (mut vao_grid, mut texture_grid, mut vbo_grid, mut ebo_grid) = (0, 0, 0, 0);
    let vert_data_grid: [f32; 20] = [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        SIZE_GRID.0 as f32,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        SIZE_GRID.1 as f32,
        0.0,
        0.0,
        1.0,
        SIZE_GRID.0 as f32,
        SIZE_GRID.1 as f32,
        0.0,
        1.0,
        1.0,
    ];
    let idxs_vert = [0u32, 1, 2, 2, 1, 3];
    unsafe {
        gl::GenVertexArrays(1, &mut vao_grid);
        // gl::GenBuffers(1, &mut tex_pos_bo);
        gl::GenBuffers(1, &mut vbo_grid);
        gl::GenBuffers(1, &mut ebo_grid);

        // VAO GRID:
        gl::BindVertexArray(vao_grid);
        {
            // VBO GRID:
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_grid);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vert_data_grid.len() * std::mem::size_of::<f32>()) as isize,
                vert_data_grid.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO GRID:
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_grid);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (idxs_vert.len() * std::mem::size_of::<u32>()) as isize,
                idxs_vert.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            texture_grid = generate_texture_for_grid();

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as i32,
                (3 * size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    (vao_grid, texture_grid)
}
