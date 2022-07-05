
pub mod display;

fn main() {
    let mut disp = display::terminal::new_terminal_display();
    display::terminal::render(&disp);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    display::terminal::draw_sprite(&mut disp, 0, 0, &[0b1010_1010,0b0101_0101,0b1010_1010]);
    display::terminal::render(&disp);
}
