//! IO backend.
//!
//! `CurlConnector` is default connector unless feature `curl_connector` is disabled and
//! feature `hyper_connector` is enabled. This behaviour will change after hyper release.

mod _base;
pub mod hyper;

pub use self::_base::Connector;
pub use self::hyper::HyperConnector;

use errors::Error;

/// Returns default connector.
///
/// See module level documentation for details.
pub fn default_connector() -> Result<Box<Connector>, Error> {
    hyper::default_connector()
}