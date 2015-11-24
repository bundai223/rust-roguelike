extern crate tcod;
extern crate rand;

use tcod::console::{Console, Root, BackgroundFlag};
use tcod::input::{Key, KeyCode};
use rand::Rng;


enum Contains {
    DoesContains,
    DoesNotContains,
}

trait Updates {
    fn update(&mut self, Key);
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

struct Bound {
    min: Point,
    max: Point
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
    fn update(&mut self, key: Key) {
        let mut movevec = Point::new( 0, 0 );
        match key {
            Key{ code: KeyCode::Left, .. }  => movevec.x = -1,
            Key{ code: KeyCode::Right, .. } => movevec.x = 1,
            Key{ code: KeyCode::Up, .. }    => movevec.y = -1,
            Key{ code: KeyCode::Down, .. }  => movevec.y = 1,
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
    fn update(&mut self, _: Key) {
        let mut rng = rand::thread_rng();

        let mut offset = Point::new(rng.gen_range(0, 3) - 1, 0);
        match WINDOW_BOUND.contains(&self.pos.offset(&offset)) {
            Contains::DoesContains => {}
            Contains::DoesNotContains => {
                offset.x = 0;
            }
        }
        offset.y = rng.gen_range(0, 3) - 1;
        match WINDOW_BOUND.contains(&self.pos.offset(&offset)) {
            Contains::DoesContains => {}
            Contains::DoesNotContains => {
                offset.y = 0;
            }
        }
        self.pos = self.pos.offset(&offset);
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.pos.x, self.pos.y, self.ascii, BackgroundFlag::Set);
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
    root.clear();
    player.render(&mut root);
    dog.render(&mut root);
    root.flush();

    while!(root.window_closed() || exit) {
        let keypress = root.wait_for_keypress(true);
        match keypress {
            Key{ code: KeyCode::Escape, .. } => exit = true,
            key => {
                player.update(key);
                dog.update(key);
            }
        }

        root.clear();
        player.render(&mut root);
        dog.render(&mut root);
        root.flush();
    }
}

fn update(objs: &mut Vec<Box<Updates>>, key: Key) {
    for i in objs.mut_iter() {
        i.update(key);
    }
}

fn render(objs: &Vec<Box<Updates>>, root: &mut Root) {
    for i in objs.iter() {
        i.render(root);
    }
}

