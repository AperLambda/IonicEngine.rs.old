extern crate gl;
extern crate glfw;

pub use glfw::{Action, Context, Glfw, Key};

pub mod system;

pub fn get_version() -> String
{
    "1.0.0".to_string()
}
