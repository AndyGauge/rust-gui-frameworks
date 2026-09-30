use accesskit::{Action, Node, NodeId, Role, Toggled};

fn main() {
    let mut checkbox = Node::new(Role::CheckBox);
    checkbox.set_label("Enable notifications");
    checkbox.set_toggled(Toggled::True);
    // Declaring supported actions is how a screen reader knows it may
    // "click" this widget on the user's behalf.
    checkbox.add_action(Action::Click);
    checkbox.add_action(Action::Focus);

    println!("{:?} toggled={:?}", checkbox.role(), checkbox.toggled());
    let _ = NodeId(2);
}
