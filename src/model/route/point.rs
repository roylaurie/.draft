use crate::model::{error::*, builder::*, identity::*, descriptor::*, route::*};

#[derive(Debug)]
pub enum Point {
    Endpoint (Endpoint),
    Junction (Junction)
}

