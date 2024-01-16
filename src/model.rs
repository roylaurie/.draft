pub mod access;
pub mod entity;
pub mod area;
pub mod world;

pub use entity::*;
pub use area::*;
pub use world::*;

#[cfg(test)]
mod tests {
    use crate::model::{self, *};
    use crate::s;

    #[test]
    fn test_manual_building() {
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
                            .key(s!("gray_cat"))
                            .name(s!("Cat"))
                            .description(s!("A gray cat"))));

        let litterbox_id = world.find_area("litter_box")
            .unwrap()
            .id();

        let cat_id = world.spawn_thing(cat, litterbox_id).unwrap();
        let cat = world.thing(cat_id).unwrap();

        assert_eq!("Cat", cat.name());

        let result = world.find_things("Cat");
        let cat = result.first().unwrap();

        assert_eq!("A gray cat", cat.description().unwrap());

        // test simple mutation

        //world.find_thing_mut("gray_cat").unwrap()
        //    .edit_description(s!("A slightly gray cat"));

        let mut cat = world.find_thing_mut("gray_cat").unwrap();
        Descriptor::editor()
            .description(s!("A slightly gray cat"))
            .edit(cat.descriptor_mut());

        let cat = world.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", cat.description().unwrap());


    }
}
