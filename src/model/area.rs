use crate::model::{types::*, builder::*, identity::*, descriptor::*, entity::*};

#[derive(Debug)]
pub struct Area {
    id: ID,
    descriptor: Descriptor,
    thing_ids: Vec<ID>,
    routes: Vec<Route>
}

pub struct AreaBuilder<'original> {
    descriptor: Option<DescriptorBuilder<'original>>,
    id: Option<u64>
}

impl<'original> AreaBuilder<'original> {
    pub fn new() -> Self {
        Self {
            descriptor: None,
            id: None
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn descriptor(&mut self, descriptor: DescriptorBuilder<'original>) -> &mut Self {
        self.descriptor = Some(descriptor);
        self
    }

    pub fn build(self) -> Area {
        Area {
            id: self.id.expect("ID not set"),
            descriptor: self.descriptor.expect("Descriptor not set").build(),
            thing_ids: Vec::new(),
            routes: Vec::new()
        }
    }
}

impl<'original> Area {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn builder() -> AreaBuilder<'original> {
        AreaBuilder::new()
    }
}

impl Descriptive for Area {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

#[derive(Debug)]
pub struct Route {
    id: ID,
    descriptor: Descriptor,
    exits: (Exit, Exit)
}

#[derive(Debug)]
pub struct Exit {
    id: ID,
    descriptor: Descriptor,
    area_id: ID
}

enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
    UpNorth,
    UpEast,
    UpSouth,
    UpWest,
    DownNorth,
    DownEast,
    DownSouth,
    DownWest
}