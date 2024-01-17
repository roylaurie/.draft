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

