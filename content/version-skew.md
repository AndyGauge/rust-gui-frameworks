The diagrams in this book show layers as if they were shared: one window library, one GPU layer, one layout engine. They are shared by **name**. Each framework pins its own version, resolved here on 30 September 2026 by generating a lockfile for the latest release of each one. Start with the window library, where the picture holds.

::code extra/version-skew.txt | label=shared layers, as resolved on 2026-09-30

Every framework that uses winit lands on 0.30.13. Floem and gpui show `-` because they carry their own windowing (a winit fork and platform layers respectively). The GPU column tells a different story: wgpu spans **six versions**, from 19.4 to 30.0.1, and Dioxus Native carries two at once. Freya shows `-` because it draws with Skia and not wgpu. Taffy shows four versions across four frameworks, AccessKit three and Parley two. Slint is the closest to current. Xilem and Dioxus Native, which share the Linebender stack, were released against versions several steps behind.

This is what drives maintenance cost. A fix in wgpu 30 does not reach a framework on wgpu 26 until that framework upgrades, and Cargo only merges semver-compatible versions, so two incompatible wgpus are compiled and linked twice. Combining two frameworks, or a framework and your own wgpu code, can put both copies in your binary. To see your own duplicates:

::inline bash | label=in your own project (illustrative, not compiled)
cargo tree -d                 # crates that appear at more than one version
cargo tree -i wgpu --depth 1  # who pulls wgpu in, and at which version
::end

The table, like the code on every page, comes from a real build. `snippets/verify.py` compiles every package and regenerates it, so it can be re-run when the ecosystem moves.
