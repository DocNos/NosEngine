use glium::*;

// Generic - "Template"
// 'a is a generic lifetime parameter. 
// In this case, the generic is the context of opengl
//      context is the current state of the window. 
pub struct _Window<'a, T: glium::glutin::ContextCurrentState>
{
    display_ : glium::Display,
    contextBuilder_ : glium::glutin::ContextBuilder<'a,T>,
    windowBuilder_ : glium::glutin::window::WindowBuilder,
    eventLoop_ : glium::glutin::event_loop::EventLoop<T>,
    
}

impl<T> _Window<T>
{
    pub fn CreateWindow(&self) -> _Window<T>
    {
        // 1. The **winit::EventsLoop** for handling events.
        let mut events_loop = 
            glium::glutin::event_loop::EventLoop::new();
        // 2. Parameters for building the Window.
        let wb = 
            glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title("Hello world");
        // 3. Parameters for building the OpenGL context.
        let cb = 
            glium::glutin::ContextBuilder::new();
        // 4. Build the Display with the given window and OpenGL context parameters and register the
        //    window with the events_loop.
        let display = 
            glium::Display::new(wb, cb, &events_loop).unwrap();
        
        let newWin : _Window<T> = _Window{ display_ : display
                                  , eventLoop_ : events_loop
                                  , windowBuilder_ : wb
                                  , contextBuilder_ : cb};
    }

    pub fn ShouldClose(&self) -> bool
    {
        
    }
}