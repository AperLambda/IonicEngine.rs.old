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

macro_rules! verify(
    ($e: expr) => {
        unsafe {
            let res = $e;
            assert_eq!(gl::GetError(), 0);
            res
        }
    }
);

pub struct Shader
{
    program: gl::types::GLuint,
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
        }
        else {
            None
        }
    }

    pub fn use_program(&mut self)
    {
        verify!(gl::UseProgram(self.program));
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
fn load_shader_program(vshader_r: &str, fshader_r: &str) -> (GLuint, GLuint, GLuint)
{
    let vshader = verify!(gl::CreateShader(gl::VERTEX_SHADER));

    let vertex_shader = CString::new(vshader_r.as_bytes()).unwrap();
    let fragment_shader = CString::new(fshader_r.as_bytes()).unwrap();

    unsafe
    {
        verify!(gl::ShaderSource(vshader, 1, &vertex_shader.as_ptr(), ptr::null()));
        verify!(gl::CompileShader(vshader));
    }
    check_shader_error(vshader);

    let fshader = verify!(gl::CreateShader(gl::FRAGMENT_SHADER));
    unsafe
    {
        verify!(gl::ShaderSource(fshader, 1, &fragment_shader.as_ptr(), ptr::null()));
        verify!(gl::CompileShader(fshader));
    }

    check_shader_error(fshader);


    let program = verify!(gl::CreateProgram());
    verify!(gl::AttachShader(program, vshader));
    verify!(gl::AttachShader(program, fshader));
    verify!(gl::LinkProgram(program));

    (program, vshader, fshader)
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
                let bytes = &bytes[.. bytes.len() - 1];
                panic!("Shader compilation failed: {}", str::from_utf8(bytes).unwrap());
            }
        }
    }
}