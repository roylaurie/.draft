use crate::{s, model::{identity::*, descriptor::*, entity::*, thing::*, area::*, access::*, builder::*}};

#[derive(Debug)]
pub struct World {
    next_id: u64,
    descriptor: Descriptor,
    areas: Vec<Area>,
    things: Vec<Thing>,
    players: Vec<Thing>, // todo: Player
    access_groups: Vec<AccessGroup>
}

pub struct WorldBuilder<'original> {
    areas: Vec<AreaBuilder<'original>>,
    next_id: u64,
}

impl<'original> WorldBuilder<'original> {
    pub fn new() -> Self {
        Self {
            areas: Vec::new(),
            next_id: 1
        }
    }

    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn area(mut self, area: AreaBuilder<'original>) -> Self {
        let area = area.id(self.generate_id());
        self.areas.push(area);
        self
    }

    pub fn build(self) -> World {
        let mut descriptor = Descriptor::builder();
        descriptor
            .name(s!("The World"))
            .description(s!("It's a brave new world"));

        World {
            next_id: self.next_id + 1,
            players: Vec::new(),
            access_groups: Vec::new(),
            descriptor: descriptor.build(),
            areas: self.areas.into_iter()
                .map(|area| area.build())
                .collect(),
            things: Vec::new(),
        }
    }
}

impl<'original> World {
    pub fn builder() -> WorldBuilder<'original> {
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

    pub fn spawn_thing(&mut self, thing: impl ThingBuilder<'original>, area_id: ID) -> Result<ID,()> {
        let mut area = self.area(area_id).expect("Area not found");
        let thing_id = self.generate_id();
        let thing = thing.id(thing_id).build_thing();
        self.things.push(thing);
        Ok(thing_id)
    }
}