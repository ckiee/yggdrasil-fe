use ed25519_dalek::PublicKey;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PeerMeta {
    // magic ascii "meta" +4
    pub major: u8,      // +1
    pub minor: u8,      // +1
    pub key: PublicKey, // +32 (::PUBLIC_KEY_LENGTH)
}
pub const PEER_META_BYTE_SIZE: usize = 38;

impl PeerMeta {
    pub fn to_bytes(&self) -> [u8; 38] {
        let mut dat: [u8; 38] = [
            0x6d, 0x65, 0x74, 0x61, self.major, self.minor, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        dat[6..].copy_from_slice(&self.key.to_bytes());
        dat
    }

    pub fn from_bytes(dat: &[u8]) -> Result<Self> {
        Ok(Self {
            major: dat[4],
            minor: dat[5],
            key: PublicKey::from_bytes(&dat[6..(6 + 32)])?,
        })
    }

    pub fn new_with_key(key: PublicKey) -> Self {
        Self {
            // v0.4
            major: 0,
            minor: 4,
            key
        }
    }
}
