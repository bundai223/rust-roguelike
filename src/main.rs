extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag};
// use tcod::console::Renderer;
use tcod::input::{KeyCode, Event, EventFlags, check_for_event};

struct Point {
    x: i32,
    y: i32,
}

struct Bound {
    min: Point,
    max: Point
}

const WindowSize: Point = Point { x: 80, y: 50 };

fn main() {
    // initialize
    let console = Bound {
        min: Point { x:0, y:0 },
        max: Point { x:WindowSize.x - 1, y:WindowSize.y - 1 },
    };
    let mut player = Point { x: 40, y: 25 };
    let mut dog    = Point { x: 10, y: 10 };

    let mut root = Root::initializer()
        .size(WindowSize.x, WindowSize.y)
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
                                player.x -= 1;
                                if player.x < console.min.x {
                                    player.x = console.min.x;
                                }
                            },
                            KeyCode::Right => {
                                player.x += 1;
                                if console.max.x < player.x {
                                    player.x = console.max.x;
                                }
                            },
                            KeyCode::Up => {
                                player.y -= 1;
                                if player.y < console.min.x {
                                    player.y = console.min.x;
                                }
                            },
                            KeyCode::Down => {
                                player.y += 1;
                                if console.max.y < player.y {
                                    player.y = console.max.y;
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        root.clear();
        render(&mut root, &player, '@');
        render(&mut root, &dog, 'd');
        root.flush();
    }
}


fn render(root: &mut Root, chara: &Point, ascii:char) {
    root.put_char(chara.x, chara.y, ascii, BackgroundFlag::Set);
}
