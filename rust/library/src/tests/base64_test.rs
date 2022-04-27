#[cfg(test)]
mod tests {
    use crate::core::encoding::base64::{base64_decode, base64_encode};

    #[test]
    fn test_base64_encode() {
        let input: &[u8] = b"hello world";
        assert_eq!(base64_encode(input), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode() {
        let input: String = "aGVsbG8gd29ybGQ=".to_string();
        assert_eq!(base64_decode(&input), "hello world".as_bytes());
    }
}
