// Code for the VAO handling and struct. Taken from https://github.com/jeaye/q3/blob/master/src/client/gfx/vbo.rs


use gl;
use gl::types::*;


#[derive(Clone)]
pub struct VBO
{
	is_bound: bool,
	name: GLuint
}

impl VBO
{
	pub fn new()
	{
		VBO
		{
			name: 0,
			is_bound: false
		}
	}

	pub fn bind()
	{

	}
}