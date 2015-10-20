extern crate tcod;
use tcod::console::{Console, Root, BackgroundFlag};
// use tcod::console::Renderer;
use tcod::input::{KeyCode, Event, EventFlags, check_for_event};


trait Updates {
    fn update(&mut self, Event);
    fn render(&self, &mut Root);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x:i32, y:i32) -> Point {
        Point {x: x, y: y }
    }
    fn offset(&self, offset:&Point) -> Point {
        Point::new(self.x + offset.x, self.y + offset.y)
    }
}

struct Player {
    ascii: char,
    pos: Point,
}

impl Player {
    fn new(ascii: char, pos: Point) -> Player {
        Player {
            ascii: ascii,
            pos: pos,
        }
    }
}

impl Updates for Player {
    fn update(&mut self, event: Event) {
        let mut movevec = Point::new( 0, 0 );
        match event {
            Event::Key(key) => {
                match key.code {
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

        match WINDOW_BOUND.contains(&self.pos.offset(&movevec)) {
            Contains::DoesContains => {
                self.pos = self.pos.offset(&movevec);
            },
            Contains::DoesNotContains => {}
        }
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.pos.x, self.pos.y, self.ascii, BackgroundFlag::Set);
    }
}

struct Npc {
    ascii: char,
    pos: Point,
}

impl Npc {
    fn new(ascii: char, pos: Point) -> Npc {
        Npc {
            ascii: ascii,
            pos: pos,
        }
    }
}

impl Updates for Npc {
    fn update(&mut self, event: Event) {
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.pos.x, self.pos.y, self.ascii, BackgroundFlag::Set);
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
        if (self.min.x <= point.x && point.x <= self.max.x) &&
           (self.min.y <= point.y && point.y <= self.max.y) {
            Contains::DoesContains
        } else {
            Contains::DoesNotContains
        }
    }
}

const WINDOW_SIZE: Point = Point { x: 80, y: 50 };
const WINDOW_BOUND: Bound = Bound {
    min: Point{ x:0, y:0 },
    max: Point{ x:WINDOW_SIZE.x - 1, y:WINDOW_SIZE.y - 1 },
};

fn main() {
    // initialize
    let mut player = Player::new('@', Point::new( 40, 25 ));
    let mut dog    = Npc::new('d', Point::new( 10, 10 ));

    let mut root = Root::initializer()
        .size(WINDOW_SIZE.x, WINDOW_SIZE.y)
        .title("Roguelike in Rustlang.")
        .init();
    let mut exit = false;

    // Render
    while!(root.window_closed() || exit) {
        // polling Event
        let checked_event = check_for_event(EventFlags::all());
        match checked_event {
            None => {},
            Some(event_tuple) => {
                match event_tuple.1 {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Escape => exit = true,
                            _ => {
                                player.update(event_tuple.1);
                                dog.update(event_tuple.1);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        root.clear();
        player.render(&mut root);
        dog.render(&mut root);
        root.flush();
    }
}


