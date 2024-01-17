use std::collections::HashMap;

pub type ID = u64;
pub type RegionID = u16;
pub type WorldID = u16;
pub type UniverseID = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Identity {
    id: ID,
    region_id: RegionID,
    world_id: WorldID,
    universe_id: UniverseID,
}

pub trait Builder: Sized {
    type Type;

    fn new() -> Self;
    fn build(self) -> Self::Type; 

    fn edit(self, original: &mut Self::Type) {
        todo!()
    }

    fn set(&mut self, field: &str, raw_value: String) -> Result<(), ()> {
        todo!()
    }

    fn validate(self) -> Self {
        self
    }
}

/// All descriptive information about and object that can be observed by a player.
/// See also its corresponding trait: `Descriptive`
#[derive(Debug)]
pub struct Descriptor {
    /// The title
    name: String,
    /// Any term that might be used to reference this
    keywords: Vec<String>,
    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    key: Option<String>,
    /// A one-liner summary. If `description` is not available, this should be used instead.
    short_description: Option<String>,
    /// A detailed and narrative description.
    description: Option<String>,
}

/// The trait that provides standard immutable access to a `Descriptor` struct
pub trait Descriptive {
    /// Fetch the `Descriptor` struct for this object
    fn descriptor(&self) -> &Descriptor;

    /// The title
    fn name(&self) -> &str {
        &self.descriptor().name
    }

    /// Any term that might be used to reference this
    fn keywords(&self) -> &Vec<String> {
        &self.descriptor().keywords
    }

    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    fn key(&self) -> Option<&String> {
        self.descriptor().key.as_ref()
    }

    /// A one-liner summary. If `description` is not available, this will be used instead.
    fn short_description(&self) -> Option<&String> {
        self.descriptor().short_description.as_ref()
    }

    /// A detailed and narrative description. If this doesn't exist, `short_description` will be used instead. 
    fn description(&self) -> Option<&String> {
        self.descriptor().description.as_ref()
            .or_else(|| self.short_description())
    }
}

pub struct DescriptorBuilder {
    name: Option<String>,
    keywords: Option<Vec<String>>,
    key: Option<String>,
    short_description: Option<String>,
    description: Option<String>
}

pub enum FieldValueType {
    String,
    Integer,
    Float,
    Boolean,
    StringArray
}

pub struct Field {
    pub name: &'static str,
    pub value_type: FieldValueType
}

#[allow(non_camel_case_types)]
pub enum DescriptorField {
    name,
    keywords,
    key,
    short_description,
    description
}

impl DescriptorField {
    pub const fn field(&self) -> Field {
        match self {
            Self::name => Field {
                name: DescriptorBuilder::FIELD_NAME,
                value_type: FieldValueType::String,
            },
            Self::keywords => Field {
                name: "keywords",
                value_type: FieldValueType::StringArray
            },
            Self::key => Field {
                name: "key",
                value_type: FieldValueType::String
            },
            Self::short_description => Field {
                name: "short_description",
                value_type: FieldValueType::String
            },
            Self::description => Field {
                name: DescriptorBuilder::FIELD_DESCRIPTION,
                value_type: FieldValueType::String
            }
        }
    }
}

impl Builder for DescriptorBuilder {
    type Type = Descriptor;

    fn new() -> Self {
        Self {
            name: None,
            keywords: None,
            key: None,
            short_description: None,
            description: None
        }
    }

    fn build(self) -> Descriptor {
        Descriptor {
            name: self.name.expect("Name not set"),
            keywords: self.keywords.unwrap_or_else(|| Vec::new()),
            key: self.key,
            short_description: self.short_description,
            description: self.description
        }
    }

    fn edit(self, original: &mut Descriptor) {
        if let Some(name) = self.name {
            original.name = name;
        }
        if self.description.is_some() {
            original.description = self.description;
        }
    }

    fn set(&mut self, field: &str, raw_value: String) -> Result<(), ()> {
        match field {
            DescriptorBuilder::FIELD_NAME => { self.name(raw_value); },
            DescriptorBuilder::FIELD_DESCRIPTION => { self.description(raw_value); },
            _ => return Err(())
        }

        Ok(())
    }
}

impl DescriptorBuilder {
    pub const FIELD_NAME: &'static str = "name";
    pub const FIELD_DESCRIPTION: &'static str = "description";

    pub fn key(&mut self, key: String) -> &mut Self {
        self.key = Some(key);
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
}

impl Descriptor {
    pub fn builder() -> DescriptorBuilder {
        DescriptorBuilder::new()
    }

    pub fn editor() -> DescriptorBuilder {
        DescriptorBuilder::new()
    }
}

impl Descriptive for Descriptor {
    fn descriptor(&self) -> &Descriptor {
        &self
    }
}

pub struct ComponentSlot {
    id: ID,
    descriptor: Descriptor,
    contents_entity_id: ID 
}

#[derive(Debug)]
pub struct InventorySlot {
    id: ID,
    descriptor: Descriptor,
    contents_entity_id: ID
}

#[derive(Debug)]
pub struct Entity {
    id: ID,
    descriptor: Descriptor,
    inventory: Vec<InventorySlot>,
    components: Vec<InventorySlot>,
}

pub struct EntityBuilder {
    id: Option<u64>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for EntityBuilder {
    type Type = Entity;

    fn new() -> Self {
        Self {
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
}

impl EntityBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn descriptor(mut self, descriptor: DescriptorBuilder) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
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

    fn id(&self) -> ID {
        self.entity().id()
    }
}

#[derive(Debug)]
pub enum Thing {
    Generic (Entity),
    Character (Character),
    Item (Item)
}

impl Descriptive for Thing {
    fn descriptor(&self) -> &Descriptor {
        match self {
            Thing::Generic(_) => todo!(),
            Thing::Character(t) => t.descriptor(),
            Thing::Item(_) => todo!(),
        }
    }
}

impl Thingy for Thing {
    fn entity(&self) -> &Entity {
        match self {
            Thing::Generic(_t) => todo!(),
            Thing::Character(t) => t.entity(),
            Thing::Item(_t) => todo!(),
        }
    }

    fn id(&self) -> ID {
        match self {
            Thing::Generic(_t) => todo!(),
            Thing::Character(t) => t.id(),
            Thing::Item(_t) => todo!(),
        }

    }
}

impl Thing {
    pub fn descriptor_mut(&mut self) -> &mut Descriptor {
        match self {
            Thing::Generic(_t) => todo!(),
            Thing::Character(t) => t.descriptor_mut(),
            Thing::Item(_t) => todo!(),
        }
    }

    pub fn edit_description(&mut self, description: String) {
        self.descriptor_mut().description = Some(description);
    }
}

#[derive(Debug)]
pub struct Character {
    entity: Entity
}

pub struct CharacterBuilder {
    id: Option<u64>,
    entity: Option<EntityBuilder>
}

impl Builder for CharacterBuilder {
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

pub trait ThingBuilder {
    fn entity(self, entity: EntityBuilder) -> Self;
    fn id(self, id: u64) -> Self;
    fn build_thing(self) -> Thing;
}

impl ThingBuilder for CharacterBuilder {
    fn entity(mut self, entity: EntityBuilder) -> Self {
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

impl Character {
    pub fn builder() -> CharacterBuilder {
        CharacterBuilder::new()
    }

    pub fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.entity.descriptor
    }
}

impl Descriptive for Character {
    fn descriptor(&self) -> &Descriptor{
        &self.entity.descriptor
    }
}

impl Thingy for Character {
    fn entity(&self) -> &Entity {
        &self.entity
    }
}

#[derive(Debug)]
pub struct Item {
    entity: Entity
}

