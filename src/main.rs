extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag};
// use tcod::console::Renderer;
use tcod::input::{KeyCode, Event, EventFlags, check_for_event};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
//     fn offset(&self, offset_x:i32, offset_y:i32) {
//         Point { x: self.x + offset_x, y: self.y + offset_y }
//     }
    fn new(x:i32, y:i32) -> Point {
        Point {x: x, y: y }
    }
    fn offset(&self, offset:&Point) -> Point {
        Point::new(self.x + offset.x, self.y + offset.y)
    }
}

struct Bound {
    min: Point,
    max: Point
}

enum Contains {
    DoesContains,
    DoesNotContains,
}

impl Bound {
    fn contains(&self, point: &Point) -> Contains {
        if self.min.x <= point.x && point.x <= self.max.x {
            if self.min.y <= point.y && point.y <= self.max.y {
                return Contains::DoesContains;
            }
        }
        return Contains::DoesNotContains;
    }
}

const WindowSize: Point = Point { x: 80, y: 50 };

fn main() {
    // initialize
    let console = Bound {
        min: Point::new( 0, 0 ),
        max: Point::new( WindowSize.x - 1, WindowSize.y - 1 ),
    };
    let mut player = Point::new( 40, 25 );
    let mut dog    = Point::new( 10, 10 );

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
        let mut movevec = Point::new( 0, 0 );
        match event {
            None => {},
            Some(event_tuple) => {
                match event_tuple.1 {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Escape => exit = true,
                            KeyCode::Left => {
                                movevec.x = -1;
                            },
                            KeyCode::Right => {
                                movevec.x = 1;
                            },
                            KeyCode::Up => {
                                movevec.y -= 1;
                            },
                            KeyCode::Down => {
                                movevec.y = 1;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
        match console.contains(&player.offset(&movevec)) {
            Contains::DoesContains => {
                player = player.offset(&movevec);
            },
            Contains::DoesNotContains => {}
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
