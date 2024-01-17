use crate::model::{identity::*, descriptor::*, entity::*};

pub trait Something: Descriptive + DescriptiveMut {
    fn entity(&self) -> &Entity;
    fn entity_mut(&mut self) -> &mut Entity;

    fn id(&self) -> ID {
        self.entity().id()
    }
}

