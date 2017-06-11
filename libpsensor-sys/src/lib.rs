#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        unsafe {
            assert!(psensor_nvidia_is_supported());
            assert!(psensor_lmsensor_is_supported());
            assert!(psensor_udisks2_is_supported());

            assert!(!psensor_amd_is_supported());
            assert!(!psensor_atasmart_is_supported());
        }
    }
}
