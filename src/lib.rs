pub use md_rs_macros::hashed_str;
mod log;
// mod obfuscation {
//     pub(crate) mod ipv4;
// }

pub fn add(left: u64, right: u64) -> u64 {
    log::log_info(&format!("Adding {left} and {right}"));
    const HASHED: &[u8; 9] = hashed_str!("example_string", 9);
    const HASHED2: &[u8; 10] = hashed_str!("EXAMPLE_STRING", 10);
    const HASHED3: &[u8; 19] = hashed_str!("example_string", 19);
    const HASHED4: &[u8; 9] = hashed_str!("example_str", 9);

    println!("{HASHED:?}");
    println!("{HASHED2:?}");
    println!("{HASHED3:?}");
    println!("{HASHED4:?}");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
