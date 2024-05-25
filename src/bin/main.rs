use std::time::{Instant};
use cow_macros::{cow_task, Resource};
use cow_ecs::comps::{Comps, CompsMut, Res, ResMut};
use cow_ecs::cow_macros::Component;
use cow_ecs::scheduler::Scheduler;
use cow_ecs::world::World;

#[derive(Component)]
pub struct Value(i32);

#[derive(Component)]
pub struct ValueSecond(i32);

#[derive(Resource)]
pub struct Poop(i32);

#[cow_task]
fn test(mut values: CompsMut<Value>) {
    for (entity, value) in values.iter() {
        value.0 += 1;
    }
}

#[cow_task]
fn show(values: Comps<Value>) {
    for (entity, value) in values.iter() {
        println!("{}", value.0);
    }
}

fn main() {
    let mut world = World::new();
    world.set_res(Poop(200));

    for i in 0..5000 {
        let entity_a = world.create();
        world.add(entity_a, Value(i));
    }

    let begin = Instant::now();
    let mut scheduler = Scheduler::new();
    scheduler.add_task(test);
    scheduler.run(&mut world);
    println!("Cow loop : {}", (Instant::now() - begin).as_millis());
}