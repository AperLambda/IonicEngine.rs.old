extern crate gl;
extern crate glfw;

use glfw::{Context};

mod system;

use system::context::{IonicContext, WindowMode};

fn main()
{
    let context = IonicContext::new();
    let mut glfw = context.get_glfw();

    let mut iwindow = context.create_window("Test Window", 800, 600, WindowMode::Windowed);
    println!("Window: Title: '{}', Width: {}, Height: {}", iwindow.get_title(), iwindow.get_size().0, iwindow.get_size().1);

    {
        let mut window = &mut iwindow.handle;

        window.set_key_polling(true);
    }

    iwindow.make_current();

    context.load_gl(&mut iwindow);

    let mut window = &mut iwindow.handle;

    while !window.should_close()
        {
            unsafe
                {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

            window.swap_buffers();
            glfw.poll_events();
        }
}
