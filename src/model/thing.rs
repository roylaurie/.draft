use crate::model::{identity::*, entity::*, character::*, item::*};

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

impl Thingy for Thing {
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

impl<'original> Thing {
    pub fn descriptor_mut(&mut self) -> &mut Descriptor {
        match self {
            Thing::Character(t) => t.descriptor_mut(),
            Thing::Item(_t) => todo!(),
        }
    }

    pub fn editor(&'original mut self) -> EntityBuilder<'original> {
        self.entity_mut().editor()
    }
}

pub trait ThingBuilder<'original> {
    fn entity(self, entity: EntityBuilder<'original>) -> Self;
    fn id(self, id: u64) -> Self;
    fn build_thing(self) -> Thing;
}