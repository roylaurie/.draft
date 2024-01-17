pub mod types {
}

pub mod error;
pub mod builder;
pub mod identity;
pub mod descriptor;
pub mod inventory;
pub mod composition;
pub mod entity;
pub mod something;
pub mod character;
pub mod item;
pub mod thing;
pub mod area;
pub mod access;
pub mod world;

pub use error::*;
pub use builder::*;
pub use identity::*;
pub use descriptor::*;
pub use inventory::*;
pub use composition::*;
pub use entity::*;
pub use something::*;
pub use character::*;
pub use item::*;
pub use thing::*;
pub use area::*;
pub use world::*;

#[cfg(test)]
mod tests {
    use crate::model::{self, *};
    use crate::s;

    #[test]
    fn test_manual_building() {
        let mut litterbox = model::Area::builder();
        litterbox.descriptor({
            let mut descriptor = model::Descriptor::creator();
            descriptor.key(s!("litter_box")).unwrap();
            descriptor.name(s!("Litter Box")).unwrap();
            descriptor.description(s!("A smelly litterbox")).unwrap();
            descriptor
        }).unwrap();

        let mut world = model::World::builder()
            .area(litterbox)
            .build().unwrap();

        dbg!(&world);

        let litterbox_id = world.find_area("litter_box")
            .unwrap()
            .id();

        let mut cat = model::Character::creator();
        cat.entity({
            let mut entity = model::Entity::creator();
            entity.descriptor({
                let mut descriptor = model::Descriptor::creator();
                descriptor.key(s!("gray_cat")).unwrap();
                descriptor.name(s!("Cat")).unwrap();
                descriptor.description(s!("A gray cat")).unwrap();
                descriptor
            }).unwrap();
            entity
        }).unwrap();

        let cat_id = world.spawn_thing(cat, litterbox_id).unwrap();
        let cat = world.thing(cat_id).unwrap();

        assert_eq!("Cat", cat.name());

        let result = world.find_things("Cat");
        let cat = result.first().unwrap();

        assert_eq!("A gray cat", cat.description().unwrap());

        // test simple mutation

        let cat = world.find_thing_mut("gray_cat").unwrap();

        let mut cat_descriptor_editor = Descriptor::editor();
        cat_descriptor_editor.description(s!("A slightly gray cat")).unwrap();
        cat_descriptor_editor.modify(cat.descriptor_mut()).unwrap();

        let cat_editor = Entity::editor();
        cat_editor.modify(cat.entity_mut()).unwrap();

        let cat = world.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", cat.description().unwrap());
    }
}
