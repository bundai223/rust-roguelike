extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag};
// use tcod::console::Renderer;
use tcod::input::{KeyCode, Event, EventFlags, check_for_event};

fn main() {
    let mut root = Root::initializer()
        .size(80, 50)
        .title("Roguelike in Rustlang.")
        .init();
//         .renderer(Renderer::GLSL)
    let mut exit = false;

    while!(root.window_closed() || exit) {
        root.clear();
        root.put_char(40, 25, '@', BackgroundFlag::Set);
        root.flush();

        let event = check_for_event(EventFlags::all());
        match event {
            None => {},
            Some(x) => {
                match x.1 {
                    Event::Key(key) => {
                        println!("key {:?}", key.code);
                        exit = true;
                    },
                    _ => {}
                }
            }
//             KeyCode::Escape => exit = true,
//             _ => {}
        }
    }
}
