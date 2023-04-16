//! # LINE Messaging API SDK for Rust (Unofficial)
//!
//! LINE Messaging API SDK for Rust is a library for developing bots using the Messaging API.
//!
//! ## Experimental🧪
//!
//! This is an experimental repository. It may not work stably.
//!
//! ## Documentation
//!
//! Official API documentation.
//!
//! - English: <https://developers.line.biz/en/docs/messaging-api/overview/>
//! - Japanese: <https://developers.line.biz/ja/docs/messaging-api/overview/>
//!
//! ## Unimplemented
//!
//! - [ ] Webhook Event Objects
//!   - [ ] Device link event
//!   - [ ] Device unlink event
//!   - [ ] LINE Things scenario execution event
//! - [ ] Message
//!   - [ ] Send narrowcast message
//!   - [ ] Get narrowcast message status
//! - [ ] Managing Audience
//! - [ ] Insight

mod awc_wrapper;
mod client;
mod error;
mod extractor;
pub mod jwt;
pub mod models;

pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::extractor::CustomHeader;
