use std::time::Instant;

use anyhow::{anyhow, ensure, Result};
use ed25519_dalek::{PublicKey, Signature, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use integer_encoding::VarInt;

#[derive(Debug, Clone)]
pub struct TreeInfo {
    root: PublicKey, // TODO: Option<> it?
    hops: Vec<TreeHop>,

    // https://matrix.to/#/!vVtVcVdzAdhGFLzFwm:matrix.org/$ogz4ipwTMgJ84lccwK_4mWarrVAJffZeC6S1VXs40e8?via=matrix.org&via=tchncs.de&via=envs.net
    // Previously we would try to sort the order of root announcements
    // received from different peers based on the timestamp that they
    // arrived.
    // rx_time: Instant, // TODO: Perhaps just move to packet meta next to type
    //
    // Then we realised that some platforms cough windows cough have horrible
    // timer resolution, so hseq was added instead, which monotonically
    // increases.
    // hseq: u64,
    /// The root announcement sequence, monotonic and
    /// sequentially incrementing in ygg-go.
    seq: u64, // like icmp's icmp_seq for the root peer's announcements
}
#[derive(Debug, Clone)]
pub struct TreeHop {
    key_next: PublicKey,
    port: u64,
    sig: Signature,
}

impl TreeInfo {
    pub fn from_bytes(dat: &[u8]) -> Result<Self> {
        let root = PublicKey::from_bytes(&dat[0..PUBLIC_KEY_LENGTH])?;
        let mut pos = PUBLIC_KEY_LENGTH + 8;
        let seq = u64::from_be_bytes(dat[PUBLIC_KEY_LENGTH..pos].try_into()?);

        let mut hops = vec![];
        // Rather annoyingly, a TreeHop#port is encoded as a varint, so
        // we just have to go at it without knowing how many hops there are.
        while pos < dat.len() {
            let key = PublicKey::from_bytes(
                &dat[pos..{
                    pos += PUBLIC_KEY_LENGTH;
                    pos
                }],
            )?;

            let (port, len) = VarInt::decode_var(&dat[pos..])
                .ok_or_else(|| anyhow!("VarInt is unexpectedly empty"))?;
            pos += len;

            let sig = Signature::from_bytes(
                &dat[pos..{
                    pos += SIGNATURE_LENGTH;
                    pos
                }],
            )?;

            hops.push(TreeHop {
                key_next: key,
                port,
                sig,
            });
        }
        // We've eaten everything now. nomnomnomnomnom <3
        assert!(pos == dat.len() - 1);

        Ok(Self { root, seq, hops })
    }
}
