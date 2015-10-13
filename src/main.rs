extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag, Renderer};
use tcod::input::KeyCode;

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

//         let keypress = Console::wait_for_keypress(true);
//        match keypress.key {
//            Special(KeyCode::Escape) => exit = true,
//            _ => {}
//        }
    }
}
