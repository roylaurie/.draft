use crate::{s, model::{error::*, identity::*, descriptor::*, entity::*, something::*, thing::*, area::*, access::*, builder::*}};

#[derive(Debug)]
pub struct World {
    identity: Identity,
    next_id: ID,
    descriptor: Descriptor,
    areas: Vec<Area>,
    things: Vec<Thing>,
    players: Vec<Thing>, // todo: Player
    access_groups: Vec<AccessGroup>
}

pub struct WorldBuilder {
    areas: Vec<AreaBuilder>,
    next_id: ID,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            areas: Vec::new(),
            next_id: 1
        }
    }

    fn generate_id(&mut self) -> ID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn area(mut self, area: AreaBuilder) -> Self {
        let area = area.id(self.generate_id());
        self.areas.push(area);
        self
    }

    pub fn build(self) -> Result<World> {
        let mut descriptor = Descriptor::creator();
        descriptor.name(s!("The World"))?;
        descriptor.description(s!("It's a brave new world"))?;

        Ok(World {
            identity: Identity::new(0, 0, 1, 1),
            next_id: self.next_id + 1,
            players: Vec::new(),
            access_groups: Vec::new(),
            descriptor: descriptor.create().unwrap(),
            areas: self.areas.into_iter()
                .map(|area| area.build())
                .collect(),
            things: Vec::new(),
        })
    }
}

impl World {
    pub fn builder() -> WorldBuilder {
        WorldBuilder::new()
    }

    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn thing(&self, id: u64) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.id() == id)
    }

    pub fn thing_mut(&mut self, id: u64) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.id() == id)
    }

    pub fn area(&self, id: u64) -> Option<&Area> {
        self.areas.iter().find(|area| area.id() == id)
    }

    pub fn find_areas(&self, query: &str) -> Vec<&Area> {
        self.areas.iter()
            .filter(|area| area.name() == query)
            .collect()
    }

    pub fn find_area(&self, key: &str) -> Option<&Area> {
        self.areas.iter().find(|area| area.key().is_some_and(|k| k == key))
    }

    pub fn find_things(&self, query: &str) -> Vec<&Thing> {
        self.things.iter()
            .filter(|thing| thing.name() == query)
            .collect()
    }

    pub fn find_thing(&self, key: &str) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.key().is_some_and(|k| k == key))
    }

    pub fn find_thing_mut(&mut self, key: &str) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.key().is_some_and(|k| k == key))
    }

    pub fn spawn_thing(&mut self, mut thing: impl ThingBuilder, area_id: ID) -> Result<ID> {
        let mut area = self.area(area_id).expect("Area not found");
        let thing_id = self.generate_id();

        thing.entity_builder().identity_builder().guid(
            thing_id,
            self.identity.region_id(),
            self.identity.world_id(),
            self.identity.universe_id())?;

        let thing = thing.build_thing()?;

        self.things.push(thing);
        Ok(thing_id)
    }
}