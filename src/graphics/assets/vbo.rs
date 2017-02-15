#![allow(unused_unsafe)]

// Code for the VAO handling and struct. Taken from https://github.com/jeaye/q3/blob/master/src/client/gfx/vbo.rs


use gl;
use gl::types::*;
use std::mem;

#[derive(Clone)]
pub struct VBO
{
    is_bound: bool,
    name: GLuint
}

impl VBO
{
    pub fn new() -> VBO
    {
        let mut vbo: GLuint = 0;
        unsafe
            {
                gl::GenBuffers(1, &mut vbo)
            }

        VBO
        {
            name: vbo,
            is_bound: false
        }
    }

    pub fn zero() -> VBO
    {
        VBO{
            name: 0,
            is_bound: false
        }
    }

    pub fn bind(&self)
    {
        unsafe
            {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.name);
            }
    }

    pub fn bind_vertex(&mut self, vertex: &[GLfloat])
    {
        if self.name != 0
        {
            unsafe
                {
                    
                    gl::BufferData(gl::ARRAY_BUFFER,
                           (vertex.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertex[0]),
                           gl::STATIC_DRAW);
                }
        }
    }
}