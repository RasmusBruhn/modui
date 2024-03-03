use winit;

pub struct EventLoop<T: Default + 'static, F>
where
    F: FnMut(winit::event::Event<T>, &winit::event_loop::EventLoopWindowTarget<T>),
{
    event_loop: winit::event_loop::EventLoop<T>,
    handlers: Vec<F>,
}

impl<F> EventLoop<(), F>
where
    F: FnMut(winit::event::Event<()>, &winit::event_loop::EventLoopWindowTarget<()>),
{
    pub fn new(handlers: Vec<F>) -> Result<Self, winit::error::EventLoopError> {
        // Create the event loop
        let event_loop = winit::event_loop::EventLoop::new()?;

        Ok(Self {
            event_loop,
            handlers,
        })
    }
}

impl<T: Default + 'static, F> EventLoop<T, F>
where
    F: FnMut(winit::event::Event<T>, &winit::event_loop::EventLoopWindowTarget<T>),
{
    pub fn from_builder(builder: &mut winit::event_loop::EventLoopBuilder<T>, handlers: Vec<F>) -> Result<Self, winit::error::EventLoopError> {
        // Create the event loop
        let event_loop = builder.build()?;

        Ok(Self {
            event_loop,
            handlers,
        })
    }
}