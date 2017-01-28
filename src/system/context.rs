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
        let window = self.glfw_context.create_window(width, height, title, window_mode).expect("Faild to create GLFW window.").0;
        IonicWindow::new(window, title)
    }

    pub fn load_gl(&self, window: &mut IonicWindow)
    {
        gl::load_with(|symbol| window.handle.get_proc_address(symbol) as *const _);
    }
}
