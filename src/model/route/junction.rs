use crate::model::{error::*, builder::*, identity::*, descriptor::*, route::*};

#[derive(Debug)]
pub struct Junction {
    entrances: Vec<Endpoint>,
    exit: Endpoint,
}

#[derive(Clone, Copy, Debug)]
pub enum JunctionField {
    Entrances,
    Exit,
}

impl JunctionField {
    pub const CLASSNAME: &'static str = "Junction";
    pub const FIELDNAME_ENTRANCES: &'static str = "entrances";
    pub const FIELDNAME_EXIT: &'static str = "exit";

    pub const FIELD_ENTRANCES: Field = Field::new(Self::FIELDNAME_ENTRANCES, FieldValueType::Object);
    pub const FIELD_EXIT: Field = Field::new(Self::FIELDNAME_EXIT, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Entrances => &Self::FIELD_ENTRANCES,
            Self::Exit => &Self::FIELD_EXIT,
        }
    }
}

impl Junction {
    pub fn entrances(&self) -> &Vec<Endpoint> {
        &self.entrances
    }

    pub fn exit(&self) -> &Endpoint {
        &self.exit
    }
}


