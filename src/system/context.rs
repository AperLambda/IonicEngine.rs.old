extern crate gl;
extern crate glfw;

pub use glfw::{Glfw, WindowMode};
use system::window::IonicWindow;

pub struct IonicContext
{
    glfw_context: Glfw
}

impl IonicContext
{
    pub fn new() -> Self
    {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        return IonicContext
            {
                glfw_context: glfw
            }
    }

    pub fn get_glfw(&self) -> Glfw
    {
        self.glfw_context
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32, window_mode: WindowMode) -> IonicWindow
    {
        let (window, events) = self.glfw_context.create_window(width, height, title, window_mode).expect("Failed to create GLFW window.");
        IonicWindow::new(window, title, events)
    }

    pub fn load_gl(&self)
    {
        gl::load_with(|symbol| self.glfw_context.get_proc_address_raw(symbol) as *const _);
    }

    // Sets the interval between two draws (default: 1)
    pub fn set_interval(&mut self, interval: u32)
    {
        self.glfw_context.set_swap_interval(glfw::SwapInterval::Sync(interval));
    }
}
