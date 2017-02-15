#![allow(unused_unsafe, dead_code)]

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::iter::repeat;
use std::marker::PhantomData;
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;
use super::gl;
use super::{GLPrimitive, GPUVec};
use gl::types::*;

pub struct Shader
{
    pub program: gl::types::GLuint,
    vshader: gl::types::GLuint,
    fshader: gl::types::GLuint
}

impl Shader
{
    pub fn new(vshader_path: &Path, fshader_path: &Path) -> Shader
    {
        let mut vshader = String::new();
        let mut fshader = String::new();

        if File::open(vshader_path).map(|mut v| v.read_to_string(&mut vshader)).is_err()
            {
                panic!("VShader cannot be loaded");
            }
        if File::open(fshader_path).map(|mut f| f.read_to_string(&mut fshader)).is_err()
            {
                panic!("FShader cannot be loaded");
            }

        let (program, vshader, fshader) = load_shader_program(&vshader[..], &fshader[..]);

        Shader
            {
                program: program,
                vshader: vshader,
                fshader: fshader
            }
    }

    pub fn get_attrib<T: GLPrimitive>(&self, name: &str) -> Option<ShaderAttribute<T>>
    {
        let c_str = CString::new(name.as_bytes()).unwrap();
        let location = unsafe { gl::GetAttribLocation(self.program, c_str.as_ptr()) };

        if unsafe { gl::GetError() } == 0 && location != -1 {
            Some(ShaderAttribute { id: location as GLuint, data_type: PhantomData })
        } else {
            None
        }
    }

    pub fn use_program(&mut self)
    {
        verify!(gl::UseProgram(self.program));
        let out_color = CString::new("out_color".as_bytes());
        verify!(gl::BindFragDataLocation(self.program, 0, out_color.unwrap().as_ptr()));
        self.bind_pos_attr();
    }

    pub fn bind_pos_attr(&mut self)
    {
        unsafe
            {
                // use the get_attrib here
                let pos_attr = gl::GetAttribLocation(self.program, CString::new("position".as_bytes()).unwrap().as_ptr());
                gl::EnableVertexAttribArray(pos_attr as GLuint);
                gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                        gl::FALSE as GLboolean, 0, ptr::null());
            }
    }

    pub fn get_uniform_location(&mut self, uniform: &str) -> i32
    {
        let name: GLint;
        unsafe {
            name = gl::GetUniformLocation(self.program, CString::new(uniform.as_bytes()).unwrap().as_ptr());
        }
        
        match name
        {
          -1 => {
                // uniform not found
                println!("Uniform not found!!");
                name
            }
          _ => { name }
        }
    }

    pub fn set_uniform_float(&mut self, uniform: i32, value: GLfloat)
    {
        unsafe
            {
                gl::Uniform1f(uniform, value);
            }
    }

    pub fn set_uniform_2_float(&mut self, uniform: i32, value1: GLfloat, value2: GLfloat)
    {
        unsafe
            {
                gl::Uniform2f(uniform, value1, value2);
            }
    }

    pub fn set_uniform_3_float(&mut self, uniform: i32, value1: GLfloat, value2: GLfloat, value3: GLfloat)
    {
        unsafe
            {
                gl::Uniform3f(uniform, value1, value2, value3);
            }
    }

    
}

pub struct ShaderAttribute<T>
{
    id: GLuint,
    data_type: PhantomData<T>
}

impl<T: GLPrimitive> ShaderAttribute<T>
{
    pub fn disable(&mut self)
    {
        verify!(gl::DisableVertexAttribArray(self.id));
    }

    pub fn enable(&mut self)
    {
        verify!(gl::EnableVertexAttribArray(self.id));
    }

    pub fn bind(&mut self, vector: &mut GPUVec<T>)
    {
        vector.bind();

        unsafe
            {
                verify!(gl::VertexAttribPointer(
                        self.id,
                        GLPrimitive::size(None::<T>) as i32,
                        GLPrimitive::gl_type(None::<T>),
                        gl::FALSE,
                        0,
                        ptr::null()));
            }
    }
    pub fn bind_sub_buffer(&mut self, vector: &mut GPUVec<T>, strides: usize, start_index: usize)
    {
        vector.bind();

        unsafe
            {
                verify!(gl::VertexAttribPointer(
                        self.id,
                        GLPrimitive::size(None::<T>) as i32,
                        GLPrimitive::gl_type(None::<T>),
                        gl::FALSE,
                        ((strides + 1) * mem::size_of::<T>()) as GLint,
                        mem::transmute(start_index * mem::size_of::<T>())));
            }
    }

    
}


/// Loads a shader program using the given vertex and fragment shader sources
pub fn load_shader_program(vshader_r: &str, fshader_r: &str) -> (GLuint, GLuint, GLuint)
{
    let vshader = create_vertex_shader(vshader_r);

    let fshader = create_fragment_shader(fshader_r);

    (create_program(vshader, fshader), vshader, fshader)
}

pub fn create_vertex_shader(vshader_r: &str) -> GLuint
{
    let vshader = verify!(gl::CreateShader(gl::VERTEX_SHADER));

    let vertex_shader = CString::new(vshader_r.as_bytes()).unwrap();

    unsafe
        {
            verify!(gl::ShaderSource(vshader, 1, &vertex_shader.as_ptr(), ptr::null()));
            verify!(gl::CompileShader(vshader));
        }
    check_shader_error(vshader);

    vshader
}

pub fn create_fragment_shader(fshader_r: &str) -> GLuint
{
    let fshader = verify!(gl::CreateShader(gl::FRAGMENT_SHADER));

    let fragment_shader = CString::new(fshader_r.as_bytes()).unwrap();

    unsafe
        {
            verify!(gl::ShaderSource(fshader, 1, &fragment_shader.as_ptr(), ptr::null()));
            verify!(gl::CompileShader(fshader));
        }

    check_shader_error(fshader);

    fshader
}

pub fn create_program(vshader: GLuint, fshader: GLuint) -> GLuint
{
    let program = verify!(gl::CreateProgram());
    verify!(gl::AttachShader(program, vshader));
    verify!(gl::AttachShader(program, fshader));
    verify!(gl::LinkProgram(program));

    program
}

/// Checks if the provided shader handle is valid
fn check_shader_error(shader: GLuint) {
    let mut compiles: i32 = 0;

    unsafe
        {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compiles);

            if compiles == 0
                {
                    println!("Shader compilation failed.");
                    let mut info_log_len = 0;

                    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_len);

                    if info_log_len > 0
                        {
                            // error check for fail to allocate memory omitted
                            let mut chars_written = 0;
                            let info_log: String = repeat(' ').take(info_log_len as usize).collect();

                            let c_str = CString::new(info_log.as_bytes()).unwrap();
                            gl::GetShaderInfoLog(shader, info_log_len, &mut chars_written, c_str.as_ptr() as *mut _);

                            let bytes = c_str.as_bytes();
                            let bytes = &bytes[..bytes.len() - 1];
                            panic!("Shader compilation failed: {}", str::from_utf8(bytes).unwrap());
                        }
                }
        }
}