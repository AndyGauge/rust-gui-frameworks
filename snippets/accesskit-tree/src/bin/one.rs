use accesskit::{Node, NodeId, Role, TreeInfo, TreeId, TreeUpdate};

const WINDOW: NodeId = NodeId(0);
const SAVE: NodeId = NodeId(1);

fn main() {
    let mut button = Node::new(Role::Button);
    button.set_label("Save");

    let mut window = Node::new(Role::Window);
    window.set_children(vec![SAVE]);

    // A description of the UI that a screen reader can walk, independent of
    // how (or whether) it is drawn.
    let update = TreeUpdate {
        nodes: vec![(WINDOW, window), (SAVE, button)],
        tree: Some(TreeInfo::new(WINDOW)),
        tree_id: TreeId::ROOT,
        focus: SAVE,
    };
    println!("{} nodes, focus on {:?}", update.nodes.len(), update.focus);
}
