pub extern crate gl;
pub extern crate glfw;
pub extern crate aperutils;
pub extern crate nalgebra as na;

pub use glfw::{Action, Context, Glfw, Key};
pub use aperutils::maths;

pub mod graphics;
pub mod system;

pub static VERSION: &'static str = "1.0.0-1";
