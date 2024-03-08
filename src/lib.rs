//! This is a simple crate to modularize winit such that ui implementations can
//! be reused

use std::fmt::Debug;
use thiserror;
use winit;

/// A modui event loop which is a wrapper around the winit event loop allowing
/// multiple module event handlers to be used once run
pub struct EventLoop<T: Default + 'static, E: Clone + Debug> {
    /// The winit event loop to use in the backend
    event_loop: winit::event_loop::EventLoop<T>,
    /// The last error which has occured, Ok if no errors has occured, used when running
    error: Result<(), EventLoopError<E>>,
}

impl<E: Clone + Debug> EventLoop<(), E> {
    /// Create a new default event loop
    pub fn new() -> Result<Self, winit::error::EventLoopError> {
        // Create the event loop
        let event_loop = winit::event_loop::EventLoop::new()?;

        Ok(Self {
            event_loop,
            error: Ok(()),
        })
    }
}

impl<T: Default + 'static, E: Clone + Debug> EventLoop<T, E> {
    /// Create a new event loop from a winit builder with a possible custom type
    ///
    /// # Parameters
    ///
    /// builder: The winit event loop builder
    pub fn from_builder(
        builder: &mut winit::event_loop::EventLoopBuilder<T>,
    ) -> Result<Self, winit::error::EventLoopError> {
        // Create the event loop
        let event_loop = builder.build()?;

        Ok(Self {
            event_loop,
            error: Ok(()),
        })
    }

    /// Retrieves a reference to the internal winit event loop
    pub fn get_event_loop(&self) -> &winit::event_loop::EventLoop<T> {
        &self.event_loop
    }

    /// Runs the event loop, for each event received, all event handlers are run
    /// on it in order until one of them returns true, an error or all handlers
    /// has been activated
    pub fn run<F>(mut self, mut event_handlers: Vec<F>) -> Result<(), EventLoopError<E>>
    where
        F: FnMut(
            &mut winit::event::Event<T>,
            &winit::event_loop::EventLoopWindowTarget<T>,
        ) -> Result<bool, E>,
    {
        self.event_loop.run(|mut event, window_target| {
            // Loop over all event handlers
            for event_handler in event_handlers.iter_mut() {
                match event_handler(&mut event, window_target) {
                    Ok(captured) => {
                        if captured {
                            break;
                        }
                    }
                    Err(error) => {
                        self.error = Err(EventLoopError::Custom(error));
                        break;
                    }
                }
            }
        })?;
        self.error
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum EventLoopError<E: Clone + Debug> {
    /// The winit event loop threw an error
    #[error("An error occured during the winit event loop: {:?}", .0)]
    Winit(String),
    /// A custom error was trown
    #[error("An user error has occured: {:?}", .0)]
    Custom(E),
}

impl<E: Clone + Debug> From<winit::error::EventLoopError> for EventLoopError<E> {
    fn from(err: winit::error::EventLoopError) -> EventLoopError<E> {
        EventLoopError::Winit(err.to_string())
    }
}
