#[cfg(target_family = "wasm")]
pub use crate::wasm_stubs::terminal_width;
#[cfg(not(target_family = "wasm"))]
use crossterm::terminal;
#[cfg(not(target_family = "wasm"))]
use tracing::debug;

#[cfg(not(target_family = "wasm"))]
const MAX_TERMINAL_WIDTH: usize = 100;
#[cfg(not(target_family = "wasm"))]
const DEFAULT_TERMINAL_WIDTH: usize = 80;

#[cfg(not(target_family = "wasm"))]
pub fn terminal_width() -> usize {
    if let Ok((cols, _)) = terminal::size() {
        let w = sanitize_width(cols as usize);
        debug!("DETECTED TERMINAL WIDTH: {cols}, it will use: {w}");
        w
    } else {
        DEFAULT_TERMINAL_WIDTH
    }
}

#[cfg(not(target_family = "wasm"))]
fn sanitize_width(width: usize) -> usize {
    match width {
        w if w > MAX_TERMINAL_WIDTH => MAX_TERMINAL_WIDTH,
        w if w < 40 => DEFAULT_TERMINAL_WIDTH,
        w => w,
    }
}
