//! Airbrake Rust is an [Airbrake][airbrake.io] notifier library for the Rust
//! Programming language. The library provides minimalist API that enables the
//! ability to send Rust errors to the Airbrake dashboard.
//!
//! Installation
//! ------------
//!
//! Add the crate to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! airbrake = "0.1"
//! ```
//!
//! Examples
//! --------
//!
//! ### Basic example
//!
//! This is the minimal example that you can use to test Airbrake Rust with your
//! project:
//!
//! ```
//! extern crate airbrake;
//!
//! use std::num::ParseIntError;
//!
//! fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
//!    number_str.parse::<i32>().map(|n| 2 * n)
//! }
//!
//! fn main() {
//!     let mut airbrake = airbrake::configure(|config| {
//!         config.project_id = "113743".to_owned();
//!         config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
//!     });
//!
//!     match double_number("NOT A NUMBER") {
//!         Ok(n) => assert_eq!(n, 20),
//!         // Asynchronously sends the error to the dashboard.
//!         Err(err) => airbrake.notify(err)
//!     }
//!
//!     // Joins worker threads.
//!     airbrake.close();
//! }
//! ```
//!
//! Configuration
//! -------------
//!
//! ### project_key & project_id
//!
//! You **must** set both `project_id` & `project_key`.
//!
//! To find your `project_id` and `project_key` navigate to your project's _General
//! Settings_ and copy the values from the right sidebar.
//!
//! ![][project-idkey]
//!
//! ```rust
//! let mut airbrake = airbrake::configure(|config| {
//!     config.project_id = "113743".to_owned();
//!     config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
//! });
//! ```
//!
//! ### host
//!
//! By default, it is set to `https://airbrake.io`. A `host` is a web address
//! containing a scheme ("http" or "https"), a host and a port. You can omit the
//! port (80 will be assumed).
//!
//! ```rust
//! let mut airbrake = airbrake::configure(|config| {
//!     config.host = "http://localhost:8080".to_owned();
//! });
//! ```
//!
//! ### workers
//!
//! The number of threads that handle notice sending. The default value is 1.
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     config.workers = 5;
//! });
//! ```
//!
//! API
//! ---
//!
//! ## airbrake
//!
//! #### airbrake.notify
//!
//! Sends an error to Airbrake *asynchronously*. `error` must implement the
//! [`std::error::Error`][stderror] trait. Returns `()`.
//!
//! ```
//! airbrake.notify(error);
//! ```
//!
//! As the second parameter, accepts a hash with additional data. That data will be
//! displayed in the _Params_ tab in your project's dashboard.
//!
//! #### airbrake.notify_sync
//!
//! Sends an error to Airbrake *synchronously*. `error` must implement the
//! [`std::error::Error`][stderror] trait. Returns `?????????`. Accepts the same
//! parameters as [`Airbrake.notify`](#airbrakenotify).
//!
//! ```
//! airbrake.notify_sync(error);
//! ```
//!
//! [airbrake.io]: https://airbrake.io
//! [notice-v3]: https://airbrake.io/docs/#create-notice-v3
//! [env_logger]: https://crates.io/crates/env_logger
//! [project-idkey]: https://s3.amazonaws.com/airbrake-github-assets/airbrake-ruby/project-id-key.png
//! [stderror]: https://doc.rust-lang.org/std/error


extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

mod config;
mod notifier;
mod notice;
mod async_sender;
mod sync_sender;

use notifier::Notifier;
use config::Config;

/// Configures an Airbrake notifier.
///
/// # Examples
///
/// ```
/// let mut airbrake = airbrake::configure(|config| {
///     config.project_id = "113743".to_owned();
///     config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
/// });
pub fn configure<F>(configurator: F) -> Notifier
    where F: Fn(&mut Config)
{
    let mut config = Config::new();
    configurator(&mut config);
    Notifier::new(config)
}
