use crate::model::{identity::*, builder::*, descriptor::*, inventory::*};


#[derive(Debug)]
pub struct Entity {
    id: ID,
    descriptor: Descriptor,
    inventory: Vec<InventorySlot>,
    components: Vec<InventorySlot>,
}

pub struct EntityBuilder<'original> {
    original: Option<&'original mut Entity>,
    id: Option<u64>,
    descriptor: Option<DescriptorBuilder<'original>>
}

impl<'original> Builder<'original> for EntityBuilder<'original> {
    type Type = Entity;

    fn new() -> Self {
        Self {
            original: None,
            id: None,
            descriptor: None
        }
    }

    fn build(self) -> Self::Type {
        Entity {
            id: self.id.expect("ID not set"),
            descriptor: self.descriptor.expect("Descriptor not set").build(),
            inventory: Vec::new(),
            components: Vec::new()
        }
    }

    fn editor(original: &'original mut Entity) -> Self {
        let mut s = Self::new();
        s.original = Some(original);
        s
    }


    fn edit(self, composite_fields_changed: Option<Vec<Field>>) -> Result<Vec<Field>, ()> {
        Ok(Vec::new())
    }
}

impl<'original> EntityBuilder<'original> {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn descriptor(mut self, descriptor: DescriptorBuilder<'original>) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
}

impl<'original> Entity {
    pub fn builder() -> EntityBuilder<'original> {
        EntityBuilder::new()
    }

    pub fn editor(&'original mut self) -> EntityBuilder<'original> {
        EntityBuilder::editor(self)
    }

    pub fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }

    pub fn id(&self) -> ID {
        self.id
    }
}

impl Descriptive for Entity {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

pub trait Thingy: Descriptive {
    fn entity(&self) -> &Entity;
    fn entity_mut(&mut self) -> &mut Entity;

    fn id(&self) -> ID {
        self.entity().id()
    }
}

