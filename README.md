
# Cow ECS - A Rust ECS library

Cow ECS is an ECS library designed with minimal features but exceptional performance, primarily intended for server use. 

Currently under development, our immediate goal is to establish a robust working API before focusing on optimizing performance.


 


## Example

```Rust


#[derive(Component)]
struct Value(i32);


#[cow_task]
fn task_to_run(mut values: CompsMut<Value>) {
    for (_, value) in values.iter_mut() {
        value.0 += 1;
        println!("Value is {}", value.0);
    }
}

fn main() {
    let mut world = World::new();
    let a = world.create();
    world.add(a, Value(1));

    let mut scheduler = Scheduler::new();
    scheduler.add_task(task_to_run);
    scheduler.run(&mut world);
}

```


## TODO

- Remove all uses of unwrap

- Parallel Scheduler



