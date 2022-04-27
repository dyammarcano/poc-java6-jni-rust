#[cfg(test)]
mod tests {
    use crate::core::encoding::base58::{base58_decode, base58_encode};

    #[test]
    fn test_base58_encode() {
        let input: &[u8] = b"hello world";
        assert_eq!(base58_encode(input), "StV1DL6CwTryKyV");
    }

    #[test]
    fn test_base58_decode() {
        let input: String = "StV1DL6CwTryKyV".to_string();
        assert_eq!(base58_decode(&input), "hello world".as_bytes());
    }
}
