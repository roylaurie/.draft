pub use crate::model::{builder::*, entity::*, descriptor::*, thing::*};

#[derive(Debug)]
pub struct Character {
    entity: Entity
}

pub struct CharacterBuilder<'original> {
    id: Option<u64>,
    entity: Option<EntityBuilder<'original>>
}

impl<'original> Builder<'original> for CharacterBuilder<'original> {
    type Type = Character;

    fn new() -> Self {
        Self {
            id: None,
            entity: None
        }
    }

    fn build(self) -> Character {
        Character {
            entity: self.entity.expect("Entity not set").build()
        }
    }
}

impl<'original> Character {
    pub fn builder() -> CharacterBuilder<'original> {
        CharacterBuilder::new()
    }
}

impl Descriptive for Character {
    fn descriptor(&self) -> &Descriptor{
        self.entity().descriptor()
    }
}

impl DescriptiveMut for Character {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        self.entity_mut().descriptor_mut()
    }
}

impl Thingy for Character {
    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl<'original> ThingBuilder<'original> for CharacterBuilder<'original> {
    fn entity(mut self, entity: EntityBuilder<'original>) -> Self {
        self.entity = Some(entity);
        self
    }

    fn id(mut self, id: u64) -> Self {
        match self.entity {
            Some(entity) => self.entity = Some(entity.id(id)),
            None => self.entity = Some(Entity::builder().id(id))
        }

        self
    }

    fn build_thing(self) -> Thing {
        Thing::Character(self.build())
    }
}

