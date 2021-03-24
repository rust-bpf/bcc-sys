use bcc_sys::bccapi::*;
use std::ffi::CString;
use std::ptr;

fn main() {
    #[cfg(any(
        feature = "v0_4_0",
        feature = "v0_5_0",
        feature = "v0_6_0",
        feature = "v0_6_1",
        feature = "v0_7_0",
        feature = "v0_8_0",
    ))]
    {
        let cs = CString::new("".to_string()).unwrap();
        let ptr = unsafe { bpf_module_create_c_from_string(cs.as_ptr(), 2, ptr::null_mut(), 0) };
        assert!(!ptr.is_null());
    }

    #[cfg(any(feature = "v0_9_0", feature = "v0_10_0",))]
    {
        let cs = CString::new("".to_string()).unwrap();
        let ptr =
            unsafe { bpf_module_create_c_from_string(cs.as_ptr(), 2, ptr::null_mut(), 0, true) };
        assert!(!ptr.is_null());
    }

    #[cfg(any(
        feature = "v0_11_0",
        feature = "v0_12_0",
        feature = "v0_13_0",
        feature = "v0_14_0",
        feature = "v0_15_0",
        feature = "v0_16_0",
        feature = "v0_17_0",
        feature = "v0_18_0",
        not(feature = "specific"),
    ))]
    {
        let cs = CString::new("".to_string()).unwrap();
        let ptr = unsafe {
            bpf_module_create_c_from_string(
                cs.as_ptr(),
                2,
                ptr::null_mut(),
                0,
                true,
                ptr::null_mut(),
            )
        };
        assert!(!ptr.is_null());
    }
}
