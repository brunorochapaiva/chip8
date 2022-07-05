const DISPLAY_ROWS: usize = 32;
const DISPLAY_COLS: usize = 64;

pub struct TerminalDisplay {
  buffer: Vec<Vec<bool>>
}

pub fn new_terminal_display() -> TerminalDisplay {
  TerminalDisplay {
    buffer: (0..DISPLAY_ROWS).map(|_| (0..DISPLAY_COLS).map(|_| false).collect()).collect()
  }
}

pub fn flip_pixel(display: &mut TerminalDisplay, row: usize, col: usize) {
  if let Some(buffer_row) = display.buffer.get_mut(row) {
    if let Some(pixel) = buffer_row.get_mut(col) {
      *pixel ^= true;
    }
  }
}

pub fn draw_sprite_row(display: &mut TerminalDisplay, row: usize, col: usize, sprite_row: u8) {
  for entry in 0..7 {
    if (0b1000_0000 >> entry) & sprite_row != 0 {
      flip_pixel(display, row, col + entry);
    }
  }
}

pub fn draw_sprite(display: &mut TerminalDisplay, row: usize, col: usize, sprite: &[u8]) {
  for (entry, sprite_row) in sprite.iter().enumerate() {
    draw_sprite_row(display, row + entry, col, *sprite_row);
  }
}

pub fn render(display: &TerminalDisplay) {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Move cursor to 1,1
  for (count, row) in display.buffer.iter().enumerate() {
    let line = row.iter().map(|&x| if x { "#".to_owned() } else { " ".to_owned() })
                     .reduce(|x, y| x + &y)
                     .unwrap_or("RENDER ERROR".to_string());
    println!("{:02}| {}", count, line);
  }
}