////////////////////////////////////////////////////////////////////////////////
///////////////////////////////// base64 ///////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/**
 * Encodes a string to base64
 *
 * @param {bytes} The bytes to encode
 * @returns {string} - The base64 encoded string
 */
pub fn base64_encode(input: &[u8]) -> String {
    let bytes = base64::encode(input);
    String::from_utf8(Vec::from(bytes)).unwrap()
}

/**
 * Decodes a base64 encoded string.
 *
 * @param {&string} The base64 encoded string
 * @return {bytes} - The decoded bytes
 */
pub fn base64_decode(input: &str) -> Vec<u8> {
    let bytes = base64::decode(input).unwrap();
    Vec::from(bytes)
}
