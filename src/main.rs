extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag};
// use tcod::console::Renderer;
use tcod::input::{KeyCode, Event, EventFlags, check_for_event};

fn main() {
    // initialize
    let console_w = 80;
    let console_h = 50;
    let mut x = 40;
    let mut y = 25;

    let mut root = Root::initializer()
        .size(console_w, console_h)
        .title("Roguelike in Rustlang.")
//         .renderer(Renderer::GLSL) // maybe need shader
        .init();
    let mut exit = false;

    // Render
    while!(root.window_closed() || exit) {
        // polling Event
        let event = check_for_event(EventFlags::all());
        match event {
            None => {},
            Some(event_tuple) => {
                match event_tuple.1 {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Escape => exit = true,
                            KeyCode::Left => {
                                x -= 1;
                                if x < 0 {
                                    x = 0;
                                }
                            },
                            KeyCode::Right => {
                                x += 1;
                                if console_w - 1 < x {
                                    x = console_w - 1;
                                }
                            },
                            KeyCode::Up => {
                                y -= 1;
                                if y < 0 {
                                    y = 0;
                                }
                            },
                            KeyCode::Down => {
                                y += 1;
                                if console_h - 1< y {
                                    y = console_h - 1;
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        render(&mut root, x, y);
    }
}


fn render(root: &mut Root, x: i32, y: i32) {
    root.clear();
    root.put_char(x, y, '@', BackgroundFlag::Set);
    root.flush();
}
