use std::ffi::CString;


use glfw::{Action, Context, Key, WindowEvent};
use glm::Mat4;
use render::render::{draw, DrawMode, Program, ToVec, Vertex, VAO, VBO};

extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod render;

fn main() {
    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_instance.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    let (mut window, events) = glfw_instance
        .create_window(500, 500, "title", glfw::WindowMode::Windowed)
        .unwrap();
    window.set_all_polling(true);
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));

    let vertices = vec![
        Vertex::new(&[0.0, 0.5, 0.0], &[1.0, 0.0, 0.0]),
        Vertex::new(&[0.5, -0.2, 0.0], &[0.0, 1.0, 0.0]),
        Vertex::new(&[-0.5, -0.2, 0.0], &[1.0, 0.0, 1.0]),
    ];

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

    let v_source = CString::new(VSST).unwrap();
    let f_source = CString::new(FSST).unwrap();

    let posisitions: Vec<f32> = vertices
        .iter()
        .map(|ver| &ver.pos)
        .map(|pos| pos.to_vec())
        .flatten()
        .collect();

    let colors: Vec<f32> = vertices
        .iter()
        .map(|ver| &ver.color)
        .map(|pos| pos.to_vec())
        .flatten()
        .collect();
    let pos_vbo = VBO::new().data(&posisitions);
    let col_vbo = VBO::new().data(&colors);

    let vao = VAO::new();
    vao.buffer(0, pos_vbo);
    vao.buffer(1, col_vbo);

    let program = Program::new(v_source, f_source);

    let uni_name = CString::new("projection").unwrap();
    let mut matrix = Mat4::identity();
    program.uniform_matrix(&uni_name, &matrix);
    let mut deg = 45;

    while !window.should_close() {
        glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    deg = 45;
                }
                WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    deg = -45;
                }
                _ => {}
            }
        }
        window.swap_buffers();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.gl_use();
        vao.bind();
        draw(DrawMode::Triangles, 0, 3);
        matrix = glm::rotate_z(&matrix, (deg as f32).to_radians() / 100.0);
        program.uniform_matrix(&uni_name, &matrix);
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
