use glfw::Window;

pub struct IonicWindow
{
    pub handle: Window,
    title: String
}

impl IonicWindow
{
    pub fn new(handle: Window, title: &str) -> Self
    {
        IonicWindow
            {
                handle: handle,
                title: title.to_string()
            }
    }

    pub fn get_title(&self) -> &str
    {
        self.title.as_ref()
    }

    pub fn get_size(&self) -> (u32, u32)
    {
        let (width, height) = self.handle.get_size();
        (width as u32, height as u32)
    }

    pub fn set_size(&mut self, width: u32, height: u32)
    {
        self.handle.set_size(width as i32, height as i32);
    }
}
