use crate::proto::TreeInfo;

pub struct PacketHandler {}

impl PacketHandler {
    pub fn handle_tree_info(tree: TreeInfo) {
        dbg!(&tree);
    }
}
