use glfw::{Context, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct IonicWindow
{
    pub handle: Window,
    title: String,
    pub events: Receiver<(f64, WindowEvent)>
}

impl IonicWindow
{
    pub fn new(handle: Window, title: &str, events: Receiver<(f64, WindowEvent)>) -> Self
    {
        IonicWindow
            {
                handle: handle,
                title: title.to_string(),
                events: events
            }
    }

    /// Gets the title of the window.
    ///
    /// WARNING: If the title was changed with glfw, it will not return the new value...
    pub fn get_title(&self) -> &str
    {
        self.title.as_ref()
    }

    /// Gets the size of the window.
    pub fn get_size(&self) -> (u32, u32)
    {
        let (width, height) = self.handle.get_size();
        (width as u32, height as u32)
    }

    /// Sets the size of the window.
    pub fn set_size(&mut self, width: u32, height: u32)
    {
        self.handle.set_size(width as i32, height as i32);
    }

    pub fn make_current(&mut self)
    {
        self.handle.make_current();
    }

    /// Swaps the front and back buffers of the window. If the swap interval is
    /// greater than zero, the GPU driver waits the specified number of screen
    /// updates before swapping the buffers.
    pub fn swap_buffers(&mut self)
    {
        self.handle.swap_buffers();
    }

    pub fn should_close(&self) -> bool
    {
        self.handle.should_close()
    }
}
