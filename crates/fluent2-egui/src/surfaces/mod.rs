//! Fluent surfaces. One file per surface; see `docs/ARCHITECTURE.md`.

mod message_bar;
mod toast;

pub use message_bar::{Intent, MessageBar, MessageBarResponse};
pub use toast::{Toast, ToastId, ToastPosition, Toaster};
