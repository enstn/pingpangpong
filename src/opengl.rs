let event_loop = EventLoop::new(); // for handling window and device events
let window = WindowBuilder::new().with_title("Test Window"); // 

// specify OpenGL attributes (tell glutin that 3.3 is the wanted OpenGL version)
let gl_context = ContextBuilder::new()
    .with_gl(GlRequest::Specific(Api::Opengl, (3, 3)))
    .build_windowed(window, &event_loop)
    .expect("Error: window context fail");

// calling thread?     
let gl_context = unsafe {
    gl_context
        .make_current()
        .expect("Failed to make context current")
};

// gl crate manages function pointers for OpenGL; so we init gl before any 
// OpenGL functions
gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

// loop to run window and only close it when CloseRequested event occurs
event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
        Event::LoopDestroyed => (),
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        }, 
        _ => (),
    }
});

// adjust window size by:
// calling resized() everytime window adjustment occurs
// pass the new window size to gl_context for viewport adjustment
WindowEvent::Resized(physical_size) => gl_context.resize(physical_size);

// OpenGL functions by drawing on a stored image; it then gets copied 
// and "swapped" -> thats why we call swap_buffers on gl_context when 
// Redraw is requested
Event::RedrawRequested(_) => {
    unsafe {
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    gl_context.swap_buffers().unwrap();
}











