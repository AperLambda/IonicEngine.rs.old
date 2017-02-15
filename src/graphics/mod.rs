extern crate aperutils;

use std::cmp;
use std::path::Path;
use self::aperutils::maths;
use self::shader::Shader;
use super::gl;
use super::gl::types::*;

#[macro_use]
pub mod macros;
pub mod shader;
pub mod assets;
pub mod primitives;

pub use self::assets::gl_primitive::GLPrimitive;
pub use self::assets::gpu_vectors::GPUVec;
pub use self::assets::vao::VAO;
pub use self::assets::vbo::VBO;

pub struct Color
{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color
{
    /// Creates a new Color who is lighter than original.
    pub fn lighter(&self, n: f32) -> Color
    {
        let i: u8 = (1.0 / (1.0 - n)) as u8;
        //let j: u8 = (255.0 / 0.7 - 255.0) as u8;
        let mut red = self.red;
        let mut green = self.green;
        let mut blue = self.blue;

        if red == 0 && green == 0 && blue == 0
            {
                return Color { red: i, green: i, blue: i, alpha: self.alpha };
            }
        if red > 0 && red < i
            {
                red = i;
            }
        if green > 0 && green < i
            {
                green = i;
            }
        if blue > 0 && blue < i
            {
                blue = i;
            }

        let mut lred: f32 = red as f32 / n;
        let mut lgreen: f32 = green as f32 / n;
        let mut lblue: f32 = blue as f32 / n;

        if lblue > 255.0
            {
                let v = blue as f32 / n - 255.0;
                if lgreen < v { lgreen += v; }
                if lred < v { lred += v; }
            } else if lgreen > 255.0
            {
                let v = green as f32 / n - 255.0;
                if lblue < v { lblue += v; }
                if lred < v { lred += v; }
            } else if lred > 255.0
            {
                let v = red as f32 / n - 255.0;
                if lgreen < v { lgreen += v; }
                if lblue < v { lblue += v; }
            }

        blue = maths::min(lblue, 255.0) as u8;
        green = maths::min(lgreen, 255.0) as u8;
        red = maths::min(lred, 255.0) as u8;

        return Color {
            red: cmp::min(red, 255),
            green: cmp::min(green, 255),
            blue: cmp::min(blue, 255),
            alpha: self.alpha
        }
    }

    /// Creates a new Color who is darker than original.
    pub fn darker(&self) -> Color
    {
        Color
            {
                red: cmp::max((self.red as f32 * 0.7) as i8, 0) as u8,
                green: cmp::max((self.green as f32 * 0.7) as i8, 0) as u8,
                blue: cmp::max((self.blue as f32 * 0.7) as i8, 0) as u8,
                alpha: self.alpha
            }
    }

    pub fn to_float(&self) -> (f32, f32, f32, f32)
    {
        (self.red as f32 / 255.0, self.green as f32 / 255.0, self.blue as f32 / 255.0, self.alpha as f32 / 255.0)
    }
}

/// The black color.
pub static COLOR_BLACK: Color = Color { red: 0, green: 0, blue: 0, alpha: 255 };
/// The white color.
pub static COLOR_WHITE: Color = Color { red: 255, green: 255, blue: 255, alpha: 255 };
/// The red color.
pub static COLOR_RED: Color = Color { red: 255, green: 0, blue: 0, alpha: 255 };
/// The green color.
pub static COLOR_GREEN: Color = Color { red: 0, green: 255, blue: 0, alpha: 255 };
/// The blue color.
pub static COLOR_BLUE: Color = Color { red: 0, green: 0, blue: 255, alpha: 255 };
/// The yellow color.
pub static COLOR_YELLOW: Color = Color { red: 255, green: 255, blue: 0, alpha: 255 };
/// The orange color.
pub static COLOR_ORANGE: Color = Color { red: 255, green: 128, blue: 0, alpha: 255 };
/// The cyan color.
pub static COLOR_CYAN: Color = Color { red: 0, green: 255, blue: 255, alpha: 255 };


/// Still in developement
#[allow(dead_code, unused_variables)]
pub struct IonicGraphics
{
    pub complement_x: f64,
    pub complement_y: f64,
    pub vao: VAO,
    pub vbo: VBO,
    pub shader: Shader
}

impl IonicGraphics
{
    pub fn new(complement_x: f64, complement_y: f64, vshader_path: &str, fshader_path: &str) -> IonicGraphics
    {
        let ionic_graphics = IonicGraphics
            {
                complement_y: complement_y,
                complement_x: complement_x,

                vao: VAO::zero(),
                vbo: VBO::zero(),
                shader: Shader::new(Path::new(vshader_path), Path::new(fshader_path))
            };
        

        ionic_graphics
    }

    pub fn bind_vao(&mut self)
    {
        self.vao = VAO::new();
        self.vao.bind();
    }

    pub fn bind_vbo(&mut self, vertex: &[GLfloat])
    {
        self.vbo = VBO::new();
        self.vbo.bind();
        self.vbo.bind_vertex(vertex);
    }

    pub fn use_program(&mut self)
    {
        self.shader.use_program();
    }

    pub fn get_uniform_location(&mut self, name: &str) -> i32
    {
        self.shader.get_uniform_location(name)
    }

    pub fn set_uniform_float(&mut self, uniform: i32, value: GLfloat)
    {
        self.shader.set_uniform_float(uniform, value);
    }

    pub fn set_uniform_2_float(&mut self, uniform: i32, value1: GLfloat, value2: GLfloat)
    {
        self.shader.set_uniform_2_float(uniform, value1, value2);
    }

    pub fn set_uniform_3_float(&mut self, uniform: i32, value1: GLfloat, value2: GLfloat, value3: GLfloat)
    {
        self.shader.set_uniform_3_float(uniform, value1, value2, value3);
    }

    /*
    pub fn set_color(color: Color)
    {
        // Some code here
    }
    */
}