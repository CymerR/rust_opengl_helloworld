use std::{ffi::{CString, CStr}, time::Duration, mem::size_of_val, ptr};

use gl::types::GLsizeiptr;
use glfw::{Action, Context, Key, WindowEvent};
use std::fs;
use glm::{Mat4, Vec3, Vec2};

use render::render::{Program, ToVec, Vertex, VAO, VBO, draw, DrawMode};

extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod render;


fn read_shader(name: &str) -> CString {
    let ad = fs::read_to_string(name).expect("Cannot read a file");
    CString::new(ad).unwrap()
}   


fn main() {
    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_instance.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    let (mut window, events) = glfw_instance
        .create_window(800, 600, "title", glfw::WindowMode::Windowed)
        .unwrap();
    window.set_all_polling(true);
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));


    let vertices:Vec<f32> = vec![
        -1.0, 1.0, 0.0,
        -1.0, -1.0, 0.0,
        1.0, 1.0, 0.0,
        -1.0, -1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
    ];

    let vbo = VBO::new()
    .data(&vertices);

    let vao = VAO::new();
    vao.buffer(0, vbo);

    let v_source = read_shader("vertex.vert");
    let f_source = read_shader("fragment.frag");


    //println!("{:?}", &v_source);

    let program = Program::new(v_source, f_source);

    let resolution =
     Vec2::new((800) as f32, (600) as f32);

    let vv_name = CString::new("res").unwrap();
    program.uniform_vec2(&vv_name, &resolution);

    let mut tval  = 0.1f32;
    let t_name = CString::new("tval").unwrap(); 
    program.uniform_float(&t_name, tval);
    while !window.should_close() {
        glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                WindowEvent::Key(key, _, Action::Press | Action::Release,_) => {
                    if key == Key::Left {
                        tval += 0.1;
                    } else if key == Key::Right {
                        tval -= 0.1;
                    }
                    if tval >= 1.0 {
                        tval = 1.0;
                    }
                    if tval <= 0.0 {
                        tval = 0.0;
                    }
                }
                _ => {}
            }
        }
        window.swap_buffers();
        
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            //gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
        }
        
        program.gl_use();
        vao.bind();
        draw(DrawMode::Triangles, 0, 6);
        
        //program.uniform_float(&t_name, tval);

        //program.uniform_matrix(&transform_name, &tranform);
        //program.uniform_matrix(&view_name, &view);

        std::thread::sleep(Duration::from_millis(16));

    }

    /*
    unsafe {
        let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw_instance.window_hint(glfw::WindowHint::ContextVersion(4, 2));
        let (mut window, event_loop) = glfw_instance
            .create_window(500, 500, "title", glfw::WindowMode::Windowed)
            .unwrap();
        window.set_all_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s));

        let vertices: Vec<f32> = vec![-0.5, -0.5, 1.0, 0.0, 0.5, 1.0, 0.5, -0.5, 1.0];
        let colors: Vec<f32> = vec![1.0, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            9 * 4 as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let mut vbo_colors = 0;
        gl::GenBuffers(1, &mut vbo_colors);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            9 * 4 as GLsizeiptr,
            colors.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::TRUE, 0, std::ptr::null());

        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::TRUE, 0, std::ptr::null());

        let VSST = "
            #version 400

            layout (location = 0) in vec3 data;
            layout (location = 1) in vec3 col;


            uniform mat4 projection;

            out vec4 fg_col;

            void main() {
                gl_Position = projection * vec4(data, 1.0f);
                fg_col = vec4(col, 1.0);
            }
        ";
        let FSST = "
            #version 400
            in vec4 fg_col;
            out vec4 colour;

            void main() {
                colour = fg_col;
            }
        ";

        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let v_source = CString::new(VSST).unwrap();
        gl::ShaderSource(vs, 1, &v_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vs);

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        let f_source = CString::new(FSST).unwrap();
        gl::ShaderSource(fs, 1, &f_source.as_ptr(), std::ptr::null());
        gl::CompileShader(fs);

        let program = gl::CreateProgram();

        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        gl::UseProgram(program);

        let name = CString::new("projection").expect("msg");
        let loc = gl::GetUniformLocation(program, name.as_ptr());

        let mut matrix = Mat4::identity();

        // matrix = glm::rotate_z(&matrix, glm::half_pi());

        gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr());

        println!("{}", &matrix);

        let mut deg = 45;
        while !window.should_close() {
            glfw_instance.poll_events();
            for (_, event) in glfw::flush_messages(&event_loop) {
                match event {
                    WindowEvent::Close
                    | WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        window.set_should_close(true);
                    }
                    _ => {}
                }
            }
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);

            gl::UseProgram(program);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            matrix = glm::rotate_z(&matrix, (deg as f32).to_radians() / 100.0);
            gl::UseProgram(program);
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr());


            window.swap_buffers();
        }
    }
    */
}
