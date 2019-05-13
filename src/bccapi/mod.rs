#[cfg(feature = "v0_4_0")]
mod v0_4_0;

#[cfg(feature = "v0_5_0")]
mod v0_5_0;

#[cfg(feature = "v0_6_0")]
mod v0_6_0;

#[cfg(feature = "v0_6_1")]
mod v0_6_1;

#[cfg(feature = "v0_7_0")]
mod v0_7_0;

#[cfg(feature = "v0_8_0")]
mod v0_8_0;

#[cfg(feature = "v0_9_0")]
mod v0_9_0;

// without a specified version, use latest version supported
#[cfg(not(any(
    feature = "v0_4_0",
    feature = "v0_5_0",
    feature = "v0_6_0",
    feature = "v0_6_1",
    feature = "v0_7_0",
    feature = "v0_8_0",
    feature = "v0_9_0",
)))]
mod v0_9_0;

#[cfg(feature = "v0_4_0")]
pub use v0_4_0::*;

#[cfg(feature = "v0_5_0")]
pub use v0_5_0::*;

#[cfg(feature = "v0_6_0")]
pub use v0_6_0::*;

#[cfg(feature = "v0_6_1")]
pub use v0_6_1::*;

#[cfg(feature = "v0_7_0")]
pub use v0_7_0::*;

#[cfg(feature = "v0_8_0")]
pub use v0_8_0::*;

#[cfg(feature = "v0_9_0")]
pub use v0_9_0::*;

// without a specified version, use latest version supported
#[cfg(not(any(
    feature = "v0_4_0",
    feature = "v0_5_0",
    feature = "v0_6_0",
    feature = "v0_6_1",
    feature = "v0_7_0",
    feature = "v0_8_0",
    feature = "v0_9_0",
)))]
pub use v0_9_0::*;
