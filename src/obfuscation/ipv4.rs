use crate::log;
use std::ffi::CString;

/// Creates a C-compatible string (CString) containing an IPv4 address from four octets.
///
/// # Arguments
///
/// * `a` - First octet of the IPv4 address
/// * `b` - Second octet of the IPv4 address
/// * `c` - Third octet of the IPv4 address
/// * `d` - Fourth octet of the IPv4 address
///
/// # Returns
///
/// * `Some(CString)` - A valid C-compatible string containing the formatted IPv4 address
/// * `None` - If the CString creation fails (e.g., due to interior null bytes).
fn generate_ipv4(a: u8, b: u8, c: u8, d: u8) -> Option<CString> {
    let ipv4_str = format!("{a}.{b}.{c}.{d}");

    match CString::new(ipv4_str) {
        Ok(ipv4_cstr) => {
            #[cfg(debug_assertions)]
            if let Ok(result) = ipv4_cstr.to_str() {
                log::log_debug(&format!("Generated IPv4 CString: {result}"));
            }
            Some(ipv4_cstr)
        }
        Err(err) => {
            log::log_error(&format!(
                "Error creating IPv4 CString with input: {a}.{b}.{c}.{d}: {err}"
            ));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ipv4(a: u8, b: u8, c: u8, d: u8, expected: &str) {
        let result = generate_ipv4(a, b, c, d).unwrap();
        assert_eq!(result.to_str().unwrap(), expected);
    }

    mod generate_ipv4_tests {
        use super::*;
        use proptest::prelude::*;

        #[test]
        fn test_raw_bytes() {
            assert_ipv4(0x00, 0x00, 0x00, 0x00, "0.0.0.0");
            assert_ipv4(0x01, 0x02, 0x03, 0x04, "1.2.3.4");
            assert_ipv4(0xaa, 0xbb, 0xcc, 0xdd, "170.187.204.221");
            assert_ipv4(0xff, 0xff, 0xff, 0xff, "255.255.255.255");
        }

        proptest! {
            #[test]
            fn test_all_addresses(a in 0u8..=255, b in 0u8..=255, c in 0u8..=255, d in 0u8..=255) {
                assert_ipv4(a,b,c,d, &format!("{a}.{b}.{c}.{d}"))
            }
        }
    }
}
