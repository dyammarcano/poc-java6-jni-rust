////////////////////////////////////////////////////////////////////////////////
///////////////////////////////// base58 ///////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/**
 * Encodes a string to base58
 *
 * @param {bytes} The bytes to encode
 * @returns {string} - The base58 encoded string
 */
pub fn base58_encode(input: &[u8]) -> String {
    base58::ToBase58::to_base58(input)
}

/**
 * Decodes a base58 encoded string.
 *
 * @param {&string} The base58 encoded string
 * @return {bytes} - The decoded bytes
 */
pub fn base58_decode(input: &str) -> Vec<u8> {
    base58::FromBase58::from_base58(input).unwrap()
}
