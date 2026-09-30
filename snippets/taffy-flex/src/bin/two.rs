use book_error::Result;
use taffy::prelude::*;

fn main() -> Result<()> {
    let mut tree: TaffyTree<&str> = TaffyTree::new();

    // A text leaf carries *context*; taffy asks us how big it wants to be.
    let label = tree.new_leaf_with_context(Style::default(), "Hello, taffy")?;
    let root = tree.new_with_children(
        Style { size: Size { width: length(100.0), height: auto() }, ..Default::default() },
        &[label],
    )?;

    tree.compute_layout_with_measure(root, Size::MAX_CONTENT, |inputs, _id, ctx, style| {
        taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, |_known, _available| match ctx {
            // A toolkit would call its text engine here (cosmic-text, parley...).
            Some(text) => Size { width: text.len() as f32 * 8.0, height: 16.0 },
            None => Size::ZERO,
        })
    })?;
    println!("label: {:?}", tree.layout(label)?.size);
    Ok(())
}
