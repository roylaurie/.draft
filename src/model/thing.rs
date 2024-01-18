use crate::model::{error::*, identity::*, descriptor::*, entity::*, something::*, character::*, item::*};

#[derive(Debug)]
pub enum Thing {
    Character (Character),
    Item (Item)
}

impl Identifiable for Thing {
    fn identity(&self) -> &Identity {
        match self {
            Thing::Character(t) => t.identity(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl IdentifiableMut for Thing {
    fn identity_mut(&mut self) -> &mut Identity {
        match self {
            Thing::Character(t) => t.identity_mut(),
            Thing::Item(t) => todo!(),
        }
    }
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

impl Exists for Thing {
    fn entity(&self) -> &Entity {
        match self {
            Thing::Character(t) => t.entity(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl ExistsMut for Thing {
    fn entity_mut(&mut self) -> &mut Entity {
        match self {
            Thing::Character(t) => t.entity_mut(),
            Thing::Item(t) => todo!(),
        }
    }
}

impl Something for Thing {}

pub trait ThingBuilder: Builder + BuildableEntity {
    fn build_thing(self) -> Result<Thing>;
}