pub type ID = u64;

pub trait Builder: Sized {
    type Type;

    fn new() -> Self;
    fn build(self) -> Self::Type; 

    fn validate(self) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct Descriptor {
    key: Option<String>,
    name: String,
    description: String
}

pub trait Descriptive {
    fn descriptor(&self) -> &Descriptor;

    fn key(&self) -> Option<&String> {
        self.descriptor().key.as_ref()
    }

    fn name(&self) -> &str {
        &self.descriptor().name
    }

    fn description(&self) -> &str {
        &self.descriptor().description
    }
}

pub struct DescriptorBuilder {
    key: Option<String>,
    name: Option<String>,
    description: Option<String>
}

impl Builder for DescriptorBuilder {
    type Type = Descriptor;

    fn new() -> Self {
        Self {
            key: None,
            name: None,
            description: None
        }
    }

    fn build(self) -> Descriptor {
        Descriptor {
            key: self.key,
            name: self.name.expect("Name not set"),
            description: self.description.expect("Description not set")
        }
    }
}

impl DescriptorBuilder {
    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl Descriptor {
    pub fn builder() -> DescriptorBuilder {
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

