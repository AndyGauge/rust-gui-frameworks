Blitz is an HTML and CSS rendering engine that does **not** embed a browser. It parses markup, resolves real CSS, lays it out and paints it, with modular parts: CSS comes from `stylo` (Servo and Firefox's style engine), layout from `taffy`, text from `parley`, accessibility from `accesskit`, and painting goes through an `anyrender` abstraction backed by Vello (GPU) or `vello_cpu`. Dioxus Native plugs Dioxus components into that engine.

::code dioxus-native-app/src/bin/one.rs

The component code is identical to desktop Dioxus. Only the launcher changes, from a webview to a native renderer. There is no JavaScript engine here: Blitz renders HTML and CSS, but it is not a web browser.

::code dioxus-native-app/src/bin/two.rs

Its lockfile shows the bill of materials: stylo, vello and vello_cpu, parley, accesskit, winit, and **two versions of wgpu** at once (0.19.4 and 26.0.1). Blitz is at 0.3.0-beta.2 (`blitz-dom` has 23 dependents) and `dioxus-native` has 4. The September 2026 Cognition announcement says investment in Dioxus Native and Blitz will increase, which is the project to watch.
