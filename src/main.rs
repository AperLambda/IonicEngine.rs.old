extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

mod system;

use system::context::IonicContext;

fn main()
{
    let context = IonicContext::new();
    let iwindow = context.create_window("Pootis", 800, 600, glfw::WindowMode::Windowed);
    println!("Window: Title: '{}', Width: {}, Height: {}", iwindow.get_title(), iwindow.get_size().0, iwindow.get_size().1);

    let mut glfw = context.get_glfw();

    let mut window = iwindow.handle;

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close()
    {
        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
    }

    //window.title = "Prout";
    //println!("{}", window.title);
}
