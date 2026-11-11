use book_error::{Error, Result};
use zgui::prelude::*;
use zgui_ui::prelude::*;
use zgui_ui_tokens::prelude::*;

// zgui-ui ports shadcn/ui. Every component reads its look from design
// tokens, so restyling the whole app is a theme, not a fork of the widgets.
#[component]
fn Deploy() -> impl IntoView {
    let branch = RwSignal::new_local("main".to_owned());
    let migrate = RwSignal::new_local(true);

    view! {
        Card {
            CardHeader {
                CardTitle {"Deploy storefront"}
                CardDescription {"Ships the branch to production."}
            }
            CardContent {
                Input(value = branch, label = "Branch")
                Switch(checked = migrate, a11y:label = "Run migrations")
            }
            CardFooter {
                Button(variant = ButtonVariant::Outline) {"Preview"}
                Button {"Deploy"}
            }
        }
    }
}

fn main() -> Result<()> {
    app()
        .with_title("Deploy")
        .run(|| view! { ThemeProvider(scheme = ColorScheme::System) { Deploy() } })
        .map_err(Error::other)
}
