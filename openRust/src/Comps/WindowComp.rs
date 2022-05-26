use glium::*;
use crate::Comps::Object::*;
// Generic - "Template"
// 'a is a lifetime parameter. 
//        Generic -> Concrete lifetime
//   Whichever parameter has the smaller lifetime become concrete,
//      any parameters with 'a written next to them must 
//      last at least as long as the concrete lifetime, EG: 
//          fn compare<'a>(p: &'a i8 , q: &'a i8) {}
//
// In this case, the generic T is the context of opengl
//      context is the current state of the window. 
// 
#[derive(Clone)]
pub struct Window<'a>//<'a, T: glium::glutin::ContextCurrentState>
{
    // INTERFACE /////////// 
    pub name_ : &'a str ,
    pub prevState_ : ObjState,
    pub currState_ : ObjState,
    pub nextState_ : ObjState,

    ////////////////////////

    display_ : glium::Display,
    //contextBuilder_ : glium::glutin::ContextBuilder, // <'a,T>
    //windowBuilder_ : glium::glutin::window::WindowBuilder,
    //eventLoop_ : glium::glutin::event_loop::EventLoop, // <T>
    
}

impl<'a> Object for Window<'a>
{
    fn Name(&self)      -> &'a str { self.name_ }
    fn PrevState(&self) -> ObjState { self.prevState_ }
    fn CurrState(&self) -> ObjState { self.currState_ }
    fn NextState(&self) -> ObjState { self.nextState_ }

    fn Create(&mut self) -> &dyn Object
    {
        // 1. The **winit::EventsLoop** for handling events.
        let mut _eventLoop = 
            glium::glutin::event_loop::EventLoop::new();
        // 2. Parameters for building the Window.
        let _windowBuilder = 
            glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title("Hello world");
        // 3. Parameters for building the OpenGL context.
        let _contextBuilder = 
            glium::glutin::ContextBuilder::new();
        // 4. Build the Display with the given window and OpenGL context parameters and register the
        //    window with the events_loop.
        self.display_ = 
            glium::Display::new(_windowBuilder
                                , _contextBuilder
                                , &_eventLoop).unwrap();
        return self;
    }

    fn CheckState(&mut self) -> ObjState { self.currState_}
    fn Update(&mut self, dt: u32) {}
    fn Destroy(&mut self) {}

}

impl<'a> Window<'a>
{
    pub fn CreateWindow(&mut self) -> &Window <'a>
    {        
        self.Create();
        self
    }

    pub fn ShouldClose(&self) -> bool
    {
        true
    }
}

