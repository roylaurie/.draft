use crate::model::{error::*, builder::*, identity::*, descriptor::*};

#[derive(Debug)]
pub struct Route {
    identity: Identity,
    descriptor: Descriptor,
    endpoints: (Endpoint, Endpoint)
}

#[derive(Debug)]
pub struct Endpoint {
    area_identity: Identity,
    area_pov_descriptor: Descriptor,
    area_pov_direction: Direction
}

#[derive(Debug)]
pub enum Direction {
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
