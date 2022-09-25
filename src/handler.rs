use crate::{
    fmt::format_key,
    proto::{PeerMeta, TreeInfo},
};
use anyhow::Result;

pub struct PacketHandler {}

impl PacketHandler {
    pub fn handle_peer_meta(&self, meta: PeerMeta) -> Result<()> {
        eprintln!(
            "Peer {} meta v{}.{}",
            format_key(&meta.key),
            meta.major,
            meta.minor
        );
        assert_eq!(meta.major, 0);
        assert_eq!(meta.minor, 4);
        Ok(())
    }

    pub fn handle_tree_info(&self, tree: TreeInfo) -> Result<()> {
        eprintln!(
            "Tree info (root = {}, hops = {}, seq = {})",
            format_key(&tree.root),
            tree.hops.len(),
            tree.seq
        );
        for hop in tree.hops {
            eprintln!("  {} {}", hop.port, hop.sig);
        }
        Ok(())
    }
}
