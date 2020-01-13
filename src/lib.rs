//! `blinds` covers up the details of your windowing for you, by providing an async API.
//!
//! A quick example of some code that prints all incoming events:
//! ```no_run
//! use blinds::{run, Event, EventStream, Key, Settings, Window};
//!
//! run(Settings::default(), app);
//!
//! async fn app(_window: Window, mut events: EventStream<Event>) {
//!     loop {
//!         while let Some(ev) = events.next_event().await {
//!             println!("{:?}", ev);
//!         }
//!     }
//! }
//! ```
//!
//! The core of blinds is [`run`], which executes your app and provides your [`Window`] and
//! [`EventStream`] instances.
//!
//! [`run`]: run()
//! [`Window`]: Window
//! [`EventStream`]: EventStream
mod event;
mod event_context;
mod event_stream;
mod run;
mod window;

pub use self::event::{
    ElementState, Event, GamepadAxis, GamepadButton, GamepadEvent, GamepadId, Key, Modifiers,
    MouseButton, MouseScrollDelta, Pointer,
};
pub use self::event_context::EventContext;
pub use self::event_stream::EventStream;
pub use self::run::{run, run_custom};
pub use self::window::{CursorIcon, Settings, Window};

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;
pub(crate) use self::window::WindowContents;
