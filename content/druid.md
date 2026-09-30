Druid (November 2018) was the Linebender team's first toolkit and, for a few years, the most visible "official" Rust GUI bet. Its design has three ideas: your app state is a value that implements `Data` (cheap to compare), widgets are generic over that state, and the tree is built once. This is the classic counter.

::code druid-counter/src/bin/one.rs

The third idea is the **Lens**. To reuse a widget on one field of a larger state, you wrap it in a lens that focuses on that field. The derive macro generates the lenses for you.

::code druid-counter/src/bin/two.rs

Druid was retired: the team's work moved to Xilem, whose `lens` function (see that page) is the direct descendant of this design. Pieces of Druid survive. `kurbo` (2D geometry) still underpins Vello, and the `piet` drawing abstraction, which Druid used with `piet-common` and cairo on Linux, has been superseded by Vello. The crate is at 0.8.3 with 29 dependents. Notice what is missing from its lockfile: winit. Druid had its own platform layer, `druid-shell`.
