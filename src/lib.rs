extern crate gl;
extern crate glfw;
extern crate aperutils;

pub use glfw::{Action, Context, Glfw, Key};
pub use aperutils::maths;

pub mod graphics;
pub mod system;

pub static VERSION: &'static str = "1.0.0-1";
