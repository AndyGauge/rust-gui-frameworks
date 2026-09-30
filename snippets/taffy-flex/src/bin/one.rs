use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let sidebar = tree.new_leaf(Style {
        size: Size { width: length(200.0), height: auto() },
        ..Default::default()
    })?;
    let content = tree.new_leaf(Style { flex_grow: 1.0, ..Default::default() })?;

    let root = tree.new_with_children(
        Style { size: Size { width: length(800.0), height: length(600.0) }, ..Default::default() },
        &[sidebar, content],
    )?;

    tree.compute_layout(root, Size::MAX_CONTENT)?;
    println!("sidebar: {:?}", tree.layout(sidebar)?.size);
    println!("content: {:?}", tree.layout(content)?.size);
    Ok(())
}
