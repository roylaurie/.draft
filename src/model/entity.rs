use crate::model::types::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Identity {
    id: ID,
    region_id: RegionID,
    world_id: WorldID,
    universe_id: UniverseID,
}

pub trait Builder<'original>: Sized {
    type Type;

    fn new() -> Self;
    fn editor(original: &'original mut Self::Type) -> Self { todo!() }
    fn build(self) -> Self::Type; 

    fn edit(self, composite_fields_changed: Option<Vec<Field>>) -> Result<Vec<Field>, ()> {
        todo!()
    }

    fn set(&mut self, field: &str, raw_value: String) -> Result<(), ()> {
        todo!()
    }

    fn validate(self) -> Self {
        self
    }
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

pub struct DescriptorBuilder<'original> {
    original: Option<&'original mut Descriptor>,
    name: Option<String>,
    keywords: Option<Vec<String>>,
    key: Option<String>,
    short_description: Option<String>,
    description: Option<String>
}

impl<'original> Builder<'original> for DescriptorBuilder<'original> {
    type Type = Descriptor;

    fn new() -> Self {
        Self {
            original: None,
            name: None,
            keywords: None,
            key: None,
            short_description: None,
            description: None
        }
    }

    fn editor(original: &'original mut Descriptor) -> Self {
        let mut s = Self::new();
        s.original = Some(original);
        s
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

    fn edit(mut self, component_fields_changed: Option<Vec<Field>>) -> Result<Vec<Field>, ()> {
        let original = self.original.unwrap();
        let mut result = Vec::new();
        if let Some(name) = self.name {
            original.name = name;
            result.push(DescriptorField::name.field());
        }
        if self.description.is_some() {
            original.description = self.description;
            result.push(DescriptorField::description.field());
        }

        Ok(result)
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

impl DescriptorBuilder<'_> {
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

impl<'original> Descriptor {
    pub fn builder() -> DescriptorBuilder<'original> {
        DescriptorBuilder::new()
    }

    pub fn editor(&'original mut self) -> DescriptorBuilder<'original> {
        DescriptorBuilder::editor(self)
    }
}

impl Descriptive for Descriptor {
    fn descriptor(&self) -> &Descriptor {
        &self
    }
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

    fn entity_mut(&mut self) -> &mut Entity {
        match self {
            Thing::Generic(_t) => todo!(),
            Thing::Character(t) => t.entity_mut(),
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

impl<'original> Thing {
    pub fn descriptor_mut(&mut self) -> &mut Descriptor {
        match self {
            Thing::Generic(_t) => todo!(),
            Thing::Character(t) => t.descriptor_mut(),
            Thing::Item(_t) => todo!(),
        }
    }

    pub fn editor(&'original mut self) -> EntityBuilder<'original> {
        self.entity_mut().editor()
    }
}

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

pub trait ThingBuilder<'original> {
    fn entity(self, entity: EntityBuilder<'original>) -> Self;
    fn id(self, id: u64) -> Self;
    fn build_thing(self) -> Thing;
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

impl<'original> Character {
    pub fn builder() -> CharacterBuilder<'original> {
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

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

#[derive(Debug)]
pub struct Item {
    entity: Entity
}

