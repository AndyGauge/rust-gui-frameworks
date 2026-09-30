use parley::{FontContext, LayoutContext, StyleProperty};

fn main() {
    let text = "Hello, parley. This paragraph wraps onto several lines.";
    let mut font_cx = FontContext::new();
    let mut layout_cx: LayoutContext<()> = LayoutContext::new();

    let mut builder = layout_cx.ranged_builder(&mut font_cx, text, 1.0, true);
    builder.push_default(StyleProperty::FontSize(16.0));
    let mut layout = builder.build(text);

    layout.break_all_lines(Some(180.0));
    println!("{} lines, {:.0} x {:.0}", layout.len(), layout.width(), layout.height());
}
