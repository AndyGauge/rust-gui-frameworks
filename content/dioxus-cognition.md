On 10 September 2026 Dioxus Labs announced that its team is **joining Cognition**, the company behind the Devin coding agent, which already used Dioxus heavily, including a terminal renderer. The Dioxus announcement describes it as the team joining, not as an acquisition, and it makes no statement about licensing or governance. The team says it will keep working on Dioxus, Blitz, Taffy and Subsecond, its hot-patching tool, and that Nico Burns will work on the open source projects full time.

For a book about shared layers, the detail that matters is **Taffy**. The same maintainers who build Dioxus Native and Blitz also maintain the layout engine underneath several other frameworks. Here is the layout dependency for Dioxus Native, straight from `cargo tree`:

::code extra/taffy-dioxus-native.txt | label=dioxus-native-app: cargo tree -i taffy

Blitz, its paint layer, its shell and Dioxus Native all route through one Taffy, plus `stylo_taffy`, which connects CSS to it. Who employs those maintainers matters beyond Dioxus, because gpui, Floem and Slint also depend on Taffy and none of them are Dioxus projects. That is the kind of concentration the ecosystem report flags for AccessKit and winit as well.

The announcement's framing is that Cognition's backing lets the team keep developing Dioxus and Blitz without financial pressure. The team had previously raised $3.5 million in venture funding. Whether that stabilises Dioxus, which is still pre-1.0 (0.7 stable, 0.8 in alpha), is the open question, and the evidence will be in the release cadence over the next year.
