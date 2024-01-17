pub mod types {
    pub type ID = u64;
    pub type RegionID = u16;
    pub type WorldID = u16;
    pub type UniverseID = u32;
}

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
        let mut litterbox = model::Area::builder();
        litterbox
            .descriptor({
                let mut descriptor = model::Descriptor::builder();
                descriptor
                    .key(s!("litter_box"))
                    .name(s!("Litter Box"))
                    .description(s!("A smelly litterbox"));
                descriptor
            });
    
        let mut world = model::World::builder()
            .area(litterbox)
            .build();

        dbg!(&world);

        let cat = model::Character::builder()
            .entity({
                let entity = model::Entity::builder();
                entity
                    .descriptor({
                        let mut descriptor = model::Descriptor::builder();
                        descriptor
                            .key(s!("gray_cat"))
                            .name(s!("Cat"))
                            .description(s!("A gray cat"));
                        descriptor
                    })
            });

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

        let mut cat_descriptor_editor = Descriptor::editor(cat.descriptor_mut());
        cat_descriptor_editor.description(s!("A slightly gray cat"));
        let fields_changed = cat_descriptor_editor.edit(None).unwrap();

        let cat_editor = Entity::editor(cat.entity_mut());
        cat_editor.edit(None/*todo*/).unwrap();

        let cat = world.find_thing("gray_cat").unwrap();
        assert_eq!("A slightly gray cat", cat.description().unwrap());


    }
}
