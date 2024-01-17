use crate::model::{identity::*, descriptor::*};

pub struct ComponentSlot {
    id: Identity,
    descriptor: Descriptor,
    contents_entity_id: ID 
}

