use crate::model::{error::*, builder::*, identity::*, descriptor::*, entity::*};

#[derive(Debug)]
pub struct Area {
    id: ID,
    descriptor: Descriptor,
    thing_ids: Vec<ID>,
    routes: Vec<Route>
}

pub struct AreaBuilder {
    descriptor: Option<DescriptorBuilder>,
    id: Option<u64>
}

impl AreaBuilder {
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

    pub fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }

    pub fn build(self) -> Area {
        Area {
            id: self.id.expect("ID not set"),
            descriptor: self.descriptor.expect("Descriptor not set").create().unwrap(),
            thing_ids: Vec::new(),
            routes: Vec::new()
        }
    }
}

impl Area {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn builder() -> AreaBuilder {
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