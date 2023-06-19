pub mod render {
    use gl::types::*;
    use nalgebra_glm::{Mat4, RealNumber};
    use std::ffi::{CString};
    use std::mem::*;

    pub struct Program {
        pid: u32,
        vertex_shader: u32,
        fragment_shader: u32,
    }
    impl Program {
        pub fn gl_use(&self) {
            unsafe {
                gl::UseProgram(self.pid);
            }
        }
        pub fn link(&self) {
            unsafe {
                gl::LinkProgram(self.pid);
            }
        }

        pub fn new(vs: CString, fs: CString) -> Self {
            unsafe {
                let vertex = gl::CreateShader(gl::VERTEX_SHADER);
                gl::ShaderSource(vertex, 1, &vs.as_ptr(), std::ptr::null());
                gl::CompileShader(vertex);

                let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
                gl::ShaderSource(fragment, 1, &fs.as_ptr(), std::ptr::null());
                gl::CompileShader(fragment);

                let program = gl::CreateProgram();
                gl::AttachShader(program, vertex);
                gl::AttachShader(program, fragment);
                gl::LinkProgram(program);
                return Self {
                    pid: program,
                    vertex_shader: vertex,
                    fragment_shader: fragment,
                };
            }
        }
        pub fn uniform_matrix(&self, name: &CString, matrix: &Mat4) {
            unsafe {
                self.gl_use();
                let loc = gl::GetUniformLocation(self.pid, name.as_ptr());
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr());
            }
        }
    }

    pub struct VBO {
        vbo: u32,
    }
    impl VBO {
        pub fn new() -> Self {
            unsafe {
                let mut vbo = 0;
                gl::GenBuffers(1, &mut vbo);
                Self { vbo }
            }
        }
        fn bind(&self) {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            }
        }
        fn buffer_data<T>(&self, data: &[T])
        where
            T: RealNumber,
        {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    size_of_val(data) as GLsizeiptr,
                    data.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        }
        pub fn data<T>(self, data: &[T]) -> Self
        where
            T: RealNumber,
        {
            self.bind();
            self.buffer_data(data);
            self
        }
    }

    pub struct VAO {
        vao: u32,
    }

    impl VAO {
        pub fn new() -> Self {
            unsafe {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao);
                Self { vao }
            }
        }
        pub fn bind(&self) {
            unsafe {
                gl::BindVertexArray(self.vao);
            }
        }
        pub fn buffer(&self, index: u32, vbo: VBO) {
            unsafe {
                self.bind();
                gl::EnableVertexAttribArray(index);
                vbo.bind();
                gl::VertexAttribPointer(index, 3, gl::FLOAT, gl::TRUE, 0, std::ptr::null());
            }
        }
    }

    pub struct Position3f {
        x: f32,
        y: f32,
        z: f32,
    }

    pub trait ToVec {
        fn to_vec(&self) -> Vec<f32>;
    }

    impl ToVec for Position3f {
        fn to_vec(&self) -> Vec<f32> {
            vec![self.x, self.y, self.z]
        }
    }

    pub struct Color(f32, f32, f32);

    impl ToVec for Color {
        fn to_vec(&self) -> Vec<f32> {
            vec![self.0, self.1, self.2]
        }
    }

    pub struct Vertex {
        pub pos: Position3f,
        pub color: Color,
    }
    impl Vertex {
        pub fn new(position: &[f32; 3], color_: &[f32; 3]) -> Self {
            let pos = Position3f {
                x: position[0],
                y: position[1],
                z: position[2],
            };
            let color = Color(color_[0], color_[1], color_[2]);
            Self { pos, color }
        }
    }

    pub enum DrawMode {
        Points,
        LineStrip,
        LineLoop,
        Lines,
        Triangles,
    }

    pub fn draw(mode: DrawMode, first: i32, count: i32) {
        unsafe {
            let glmode = match mode {
                DrawMode::Points => gl::POINTS,
                DrawMode::LineStrip => gl::LINE_STRIP,
                DrawMode::LineLoop => gl::LINE_LOOP,
                DrawMode::Lines => gl::LINES,
                DrawMode::Triangles => gl::TRIANGLES,
            };
            gl::DrawArrays(glmode, first, count);
        }
    }
}
