use evenio::prelude::*;

#[derive(Event)]
struct Init;

#[derive(Component)]
struct Foo;

fn handler1(receiver: Receiver<Init>, mut sender: Sender<(Spawn, Insert<Foo>)>) {
    let a = sender.spawn();
    sender.insert(a, Foo);
    let b = sender.spawn();
    dbg!(b);
}

fn handler2(receiver: Receiver<Insert<Foo>, ()>, mut sender: Sender<Spawn>) {
    let c = sender.spawn();
    dbg!(c);
}

fn main() {
    let mut world = World::new();
    world.add_handler(handler1);
    world.add_handler(handler2);
    world.send(Init);
}
