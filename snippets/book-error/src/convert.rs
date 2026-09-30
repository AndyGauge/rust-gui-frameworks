//! `From` impls: these are what make `?` work across crate boundaries.

use crate::Error;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

macro_rules! from {
    ($feature:literal, $ty:ty => $variant:ident) => {
        #[cfg(feature = $feature)]
        impl From<$ty> for Error {
            fn from(e: $ty) -> Self {
                Error::$variant(e)
            }
        }
    };
}

from!("semver", semver::Error => Semver);
from!("glutin", glutin::error::Error => Gl);
from!("winit", winit::error::EventLoopError => EventLoop);
from!("winit", winit::error::OsError => Os);
from!("raw-window-handle", raw_window_handle::HandleError => Handle);
from!("wgpu", wgpu::RequestAdapterError => Adapter);
from!("wgpu", wgpu::RequestDeviceError => Device);
from!("taffy", taffy::TaffyError => Layout);
from!("vello", vello::Error => Render);
from!("ab-glyph", ab_glyph::InvalidFont => Font);
from!("slint", slint::PlatformError => Platform);
from!("tauri", tauri::Error => Tauri);
from!("iced", iced::Error => Iced);
from!("eframe", eframe::Error => Eframe);

// JavaScript exceptions cross the wasm boundary as `JsValue`, in both directions.
#[cfg(feature = "wasm")]
impl From<wasm_bindgen::JsValue> for Error {
    fn from(v: wasm_bindgen::JsValue) -> Self {
        Error::Js(v.as_string().unwrap_or_else(|| format!("{v:?}")))
    }
}

#[cfg(feature = "wasm")]
impl From<Error> for wasm_bindgen::JsValue {
    fn from(e: Error) -> Self {
        wasm_bindgen::JsValue::from_str(&e.report())
    }
}
