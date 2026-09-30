use parley::{FontContext, FontWeight, LayoutContext, StyleProperty};

fn main() {
    let text = "Bold start, then regular text.";
    let mut font_cx = FontContext::new();
    let mut layout_cx: LayoutContext<()> = LayoutContext::new();

    let mut builder = layout_cx.ranged_builder(&mut font_cx, text, 1.0, true);
    builder.push_default(StyleProperty::FontSize(18.0));
    // Style a byte range; the builder keeps spans separate from the text.
    builder.push(StyleProperty::FontWeight(FontWeight::BOLD), 0..10);
    let mut layout = builder.build(text);
    layout.break_all_lines(None);

    // Each line is a sequence of runs of same-font glyphs, ready for a renderer.
    for line in layout.lines() {
        println!("line with {} items", line.items().count());
    }
}
