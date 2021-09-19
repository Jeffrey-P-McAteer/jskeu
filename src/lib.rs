
/**
 * Each supported OS gets a directory under src/<os-name>/mod.rs
 * which gets exported as jskeu_os to clients.
 * The goal is to abstract over fullscreen OS graphics
 * and provide a unified graphics/app/environment api surface.
 */

#[cfg(target_os="linux")]
pub mod linux;
#[cfg(target_os="linux")]
pub use linux as jskeu_os;

#[cfg(target_os="windows")]
pub mod windows;
#[cfg(target_os="windows")]
pub use windows as jskeu_os;

#[cfg(target_os="macos")]
pub mod macos;
#[cfg(target_os="macos")]
pub use macos as jskeu_os;


pub mod util;


