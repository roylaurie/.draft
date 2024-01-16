use elserpg::model::{self, *};
use elserpg::s;

fn main() {
    let litterbox = model::Area::builder()
        .descriptor(model::Descriptor::builder()
            .key(s!("litter_box"))
            .name(s!("Litter Box"))
            .description(s!("A smelly litterbox")));
 
    let mut world = model::World::builder()
        .area(litterbox)
        .build();

    dbg!(&world);

    let cat = model::Character::builder()
        .entity(
            model::Entity::builder()
                .descriptor(
                    model::Descriptor::builder()
                        .name(s!("Cat"))
                        .description(s!("A gray cat"))));

    let litterbox_id = world.find_area("litter_box")
        .unwrap()
        .id();

    let cat_id = world.spawn_thing(cat, litterbox_id).unwrap();
    let cat = world.thing(cat_id).unwrap();

    print!("You see {}.", cat.name());

    let result = world.find_things("Cat");
    let cat = result.first().unwrap();

    println!(" {}", cat.description());
}
