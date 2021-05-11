
#[derive(Debug)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn update_y(&mut self) {
        self.y += 1;
    }

    fn update_x(&mut self) {
        self.x += 1;
    }

    fn update(&mut self) {
        self.update_y();
        self.update_x();
    }
}

fn main() {
    let mut i = 1;
    i += 1;
    println!("i = {}", i);

    let mut pos = Position {y:1, x:1};
    pos.x = 2;
    let pos2 = &mut pos; // effectively either pos or pos2 should take over now - can't use both, we will use pos
    pos2.y = 2;
    println!("pos = {:?}", pos);

    pos.update();
    println!("pos = {:?}", pos);
    pos.update_x();
    println!("pos = {:?}", pos);
    pos.update_y();
    println!("pos = {:?}", pos);
    Position::update(&mut pos);
    println!("pos = {:?}", pos);

}
