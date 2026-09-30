Before wgpu, getting pixels on screen from a Rust program meant OpenGL, and OpenGL needs a *context*: the object your driver uses to remember drawing state. glutin creates those contexts. Notably it does **not** create windows. You describe the framebuffer you want, and the platform finds the closest match.

::code glutin-gl/src/bin/one.rs

That separation of concerns is the pattern the rest of the stack copies. glutin is handed a raw display handle, the same kind of handle any window library can produce, and turns it into an OpenGL display. On a Mac that means Apple's CGL, which is what this snippet printed when run.

::code glutin-gl/src/bin/two.rs

glutin is now a supporting character. egui's `eframe` still depends on it for its OpenGL (glow) backend, but newer toolkits mostly target wgpu, which is why it sits at 260 dependents rather than thousands. It is a good example of a primitive that did its job, and whose job has been absorbed by a higher-level one.
