pub use crate::timestamp::*;
pub use crate::types::*;

mod mods;
pub use self::mods::*;

pub mod requests;

#[cfg(feature = "reqwest_blocking")]
pub use self::requests::default_client;

use std::error;

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an get API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn get<S>(&self, method_url: S, params: &[(&str, &str)]) -> Result<String, Self::Error>
    where
        S: AsRef<str> + Send;

    /// Make an post API call to Slack. Takes a map of parameters that get appended to the request as body
    /// and a map of headers to set.
    fn post<S>(
        &self,
        method_url: S,
        form: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<String, Self::Error>
    where
        S: AsRef<str> + Send;
}
