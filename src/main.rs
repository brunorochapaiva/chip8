
pub mod display;
pub mod memory;

fn main() {
    let mut disp = display::terminal::new_terminal_display();
    disp.render();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    disp.draw_sprite(0, 0, &[0b1010_1010,0b0101_0101,0b1010_1010]);
    disp.render();
}
