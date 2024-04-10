use std::rc::Rc;

use evenio::prelude::*;

#[derive(Event)]
struct Test;

fn main() {
    let mut world = World::new();
    let x = Rc::new(123);
    world.add_handler(move |_receiver: Receiver<Test>| {
        println!("{x:?}");
    });
}
