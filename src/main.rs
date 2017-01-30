extern crate gl;
extern crate glfw;
extern crate ionic_engine;

use ionic_engine::system::context::{IonicContext, WindowMode};
use ionic_engine::graphics::*;

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

    context.load_gl();

    let clear_color = COLOR_BLACK.to_float();

    while !iwindow.should_close()
        {
            unsafe
                {
                    gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

            iwindow.swap_buffers();
            glfw.poll_events();
        }
}
