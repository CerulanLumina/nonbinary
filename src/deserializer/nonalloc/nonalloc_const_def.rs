
//! Just the worst code.

/// Retrieve the environment variable NB_ARRAY_SIZE as a usize. This is const.
pub const fn max_array_size() -> usize {
    if let Some(val) = option_env!("NB_ARRAY_SIZE") {
        parse_usize(val)
    } else {
        64
    }
}

/// Hacky str to usize parsing
const fn parse_usize(str: &str) -> usize {
    let mut val: usize = 0;
    let mut end = str.len();
    assert!(end <= 8, "usize environment variable too long");
    let bytes = str.as_bytes();
    let mut count: u32 = 0;
    while end > 0 {
        end -= 1;
        let cur_byte = bytes[end];
        assert!(cur_byte >= 48 && cur_byte <= 58, "Not a number");
        val += (bytes[end] as usize - 48) * 10usize.pow(count);
        count += 1;
    }
    val
}

#[cfg(all(test, feature = "dumb-tests"))]
mod test {
    use crate::deserializer::nonalloc::nonalloc_const_def::max_array_size;

    #[test]
    fn why() {
        let owo = [0; max_array_size()];
        assert_eq!(owo.len(), 32);
    }

}

