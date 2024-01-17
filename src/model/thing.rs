use crate::model::{error::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*};

#[derive(Debug)]
pub enum Thing {
    Character (Character),
    Item (Item)
}

impl Descriptive for Thing {
    fn descriptor(&self) -> &Descriptor {
        match self {
            Thing::Character(t) => t.descriptor(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl DescriptiveMut for Thing {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        match self {
            Thing::Character(t) => t.descriptor_mut(),
            Thing::Item(t) => todo!(),
        }

    }
}

impl Something for Thing {
    fn entity(&self) -> &Entity {
        match self {
            Thing::Character(t) => t.entity(),
            Thing::Item(_t) => todo!(),
        }
    }

    fn entity_mut(&mut self) -> &mut Entity {
        match self {
            Thing::Character(t) => t.entity_mut(),
            Thing::Item(_t) => todo!(),
        }
    }


    fn id(&self) -> ID {
        match self {
            Thing::Character(t) => t.id(),
            Thing::Item(_t) => todo!(),
        }

    }
}

pub trait ThingBuilder: Builder {
    fn id(&mut self, id: ID) -> Result<()>;
    fn entity(&mut self, entity: EntityBuilder) -> Result<()>;
    fn build_thing(self) -> Result<Thing>;
}