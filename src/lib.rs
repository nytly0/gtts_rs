//! gtts_rs: A Rust library for Google Translate's text-to-speech API.
//!
//! # Examples
//!
//! ```rust
//! use gtts_rs::tts::GttsClient;
//!
//! fn main() {
//!     let tts = GttsClient::default();
//!     tts.speak("Hello, world!").unwrap();
//! }
//! ```
mod constants;

pub mod lang;
pub mod tts;
pub mod url;