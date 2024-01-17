use crate::model::{identity::*, descriptor::*};

#[derive(Debug)]
pub struct InventorySlot {
    id: ID,
    descriptor: Descriptor,
    contents_entity_id: ID
}

