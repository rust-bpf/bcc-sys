cfg_if! {
    if #[cfg(feature = "v0_4_0")] {
        include!("v0_4_0.rs");
    } else if #[cfg(feature = "v0_5_0")] {
        include!("v0_5_0.rs");
    } else if #[cfg(feature = "v0_6_0")] {
        include!("v0_6_0.rs");
    } else if #[cfg(feature = "v0_6_1")] {
        include!("v0_6_1.rs");
    } else if #[cfg(feature = "v0_7_0")] {
        include!("v0_7_0.rs");
    } else if #[cfg(feature = "v0_8_0")] {
        include!("v0_8_0.rs");
    } else {
        include!("v0_9_0.rs");
    }
}
