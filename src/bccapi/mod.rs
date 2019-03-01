mod v0_4_0;
mod v0_5_0;
mod v0_6_0;
mod v0_6_1;
mod v0_7_0;
mod v0_8_0;

#[cfg(feature = "0.4.0")]
pub use v0_4_0::*;

#[cfg(feature = "0.5.0")]
pub use v0_5_0::*;

#[cfg(feature = "0.6.0")]
pub use v0_6_0::*;

#[cfg(feature = "0.6.1")]
pub use v0_6_1::*;

#[cfg(feature = "0.7.0")]
pub use v0_7_0::*;

#[cfg(feature = "0.8.0")]
pub use v0_8_0::*;

#[cfg(not(any(
    feature = "0.4.0",
    feature = "0.5.0",
    feature = "0.6.0",
    feature = "0.6.1",
    feature = "0.7.0",
    feature = "0.8.0"
)))]
pub use v0_8_0::*;
