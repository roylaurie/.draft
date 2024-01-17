use crate::{s, model::{error::*, identity::*, builder::*, descriptor::*, inventory::*}};


#[derive(Debug)]
pub struct Entity {
    id: ID,
    descriptor: Descriptor,
    inventory: Vec<InventorySlot>,
    components: Vec<InventorySlot>,
}

pub struct EntityBuilder {
    builder_mode: BuilderMode,
    id: Option<ID>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for EntityBuilder {
    type Type = Entity;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            id: None,
            descriptor: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator()
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn create(self) -> Result<Self::Type> {
        Ok(Entity {
            id: self.id
                .ok_or_else(|| Error::FieldNotSet{class: "Entity", field: "id"} )?,
            descriptor: self.descriptor
                .ok_or_else(|| Error::FieldNotSet{class: "Entity", field: "descriptor"} )?
                .create()?,
            inventory: Vec::new(),
            components: Vec::new()
        })
    }


    fn modify(self, original: &mut Entity) -> Result<ModifyResult> {
        Ok(ModifyResult::new(Vec::new()))
    }
}

impl EntityBuilder {
    pub fn id(mut self, id: u64) -> Result<()> {
        self.id = Some(id);
        Ok(())
    }

    pub fn descriptor(mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }
}

impl Build for Entity {
    type BuilderType = EntityBuilder;
}

impl Entity {
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

impl DescriptiveMut for Entity {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

