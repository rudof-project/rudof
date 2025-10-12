use crossterm::terminal;

const MAX_TERMINAL_WIDTH: usize = 150;
const DEFAULT_TERMINAL_WIDTH: usize = 80;

pub fn terminal_width() -> usize {
    if let Ok((cols, _)) = terminal::size() {
        return sanitize_width(cols as usize);
    } else {
        DEFAULT_TERMINAL_WIDTH
    }
}

fn sanitize_width(width: usize) -> usize {
    match width {
        w if w > MAX_TERMINAL_WIDTH => MAX_TERMINAL_WIDTH,
        w if w < 40 => DEFAULT_TERMINAL_WIDTH,
        w => w,
    }
}
