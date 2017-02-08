#![allow(unused_unsafe)]

// Code for the VAO handling and struct. Taken from https://github.com/jeaye/q3/blob/master/src/client/gfx/vao.rs


use gl;
use gl::types::*;


#[derive(Clone)]
pub struct VAO
{
    is_bound: bool,
    name: GLuint
    
}

impl VAO
{
    pub fn new() -> VAO
    {
        let mut name: GLuint = 0;
        unsafe
            {
                gl::GenVertexArrays(1, &mut name);
            }
        if name <= 0
        {
            panic!("Couldn't generate VAO!");
        }
        VAO
        {
            name: name,
            is_bound: false
        }
    }

    pub fn zero() -> VAO
    {
        VAO
        {
            name: 0,
            is_bound: false
        }
    }

    pub fn bind(&mut self)
    {
        if self.name <= 0
        {
            panic!("VAO not generated!")
        }

        unsafe
            {
                verify!(gl::BindVertexArray(self.name));
                self.is_bound = true;
            }
    }

    pub fn unbind(&self)
    {
        if self.name <= 0
        {
            panic!("VAO not generated!")
        }

        unsafe
            {
                verify!(gl::BindVertexArray(0));
            }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: GLuint)
    {
        // TODO: check if self.is_bound is true
        unsafe
            {
                // TODO: check if the next function completed successfully
                gl::EnableVertexAttribArray(attrib);
            }
        
    }

    pub fn vertex_attrib_pointer_f32(&self, index: GLuint, size: GLint,
                                    normalized: bool, stride: GLsizei, ptr: *const GLvoid)
    {
        // TODO: check if self.is_bound is true
        unsafe
            {
                // TODO: check if the next function completed successfully

                gl::VertexAttribPointer(index, size, gl::FLOAT, normalized as u8, stride, ptr);
            }
    }
}


impl Drop for VAO
{
    fn drop(&mut self)
    {
        if self.name != 0
        {
            unsafe
                {
                    // TODO: check if the function completed successfully
                    gl::DeleteVertexArrays(1, &self.name);
                }
        }
    }
}