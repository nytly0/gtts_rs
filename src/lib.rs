//! gtts_rs: A simple and easy text-to-speech module for your needs using
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
