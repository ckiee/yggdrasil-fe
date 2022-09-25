use ed25519_dalek::PublicKey;

pub fn format_byte_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|x| format!("{x:0>2x}"))
        .collect::<String>()
}

pub fn format_key(key: &PublicKey) -> String {
    format_byte_string(&key.to_bytes())
}
