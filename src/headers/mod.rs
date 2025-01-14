pub use self::utils::EmailWriter;

mod hex;
mod hex_encoding;
mod rfc2047;
pub mod rfc2231;
mod utils;

pub(super) const MAX_LINE_LEN: usize = 76;
