use glfw::Context;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    
    let (mut window, events) = glfw
        .create_window(
            1200,
            600,
            "Evosector",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window");
    
    window.make_current();
    window.set_all_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut imgui = ImContext::create();
    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
        }

        let ui = imgui_glfw.frame(&mut window, &mut imgui);

        ui.show_demo_window(&mut true);

        imgui_glfw.draw(ui, &mut window);

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}