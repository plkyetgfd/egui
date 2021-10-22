//! Miscellaneous tools used by the rest of egui.

pub mod cache;
pub(crate) mod fixed_cache;
mod history;
pub mod id_any_map;
pub mod undoer;

pub use history::History;
pub use id_any_map::IdAnyMap;

pub use epaint::util::{hash, hash_with};
