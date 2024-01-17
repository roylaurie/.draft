use crate::{s, model::{error::*, builder::*}};

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

pub trait Ident {
    fn identity(&self) -> &Identity;

    fn id(&self) -> ID {
        self.identity().id
    }

    fn region_id(&self) -> RegionID {
        self.identity().region_id
    }

    fn world_id(&self) -> WorldID {
        self.identity().world_id
    }

    fn universe_id(&self) -> UniverseID {
        self.identity().universe_id
    }

}

pub trait IdentMut: Ident {
    fn identity_mut(&mut self) -> &mut Identity;
}

impl Ident for Identity {
    fn identity(&self) -> &Identity {
        self
    }
}

impl IdentMut for Identity {
    fn identity_mut(&mut self) -> &mut Identity {
        self
    }
}

pub enum IdentityField {
    ID,
    RegionID,
    WorldID,
    UniverseID,
}

impl IdentityField {
    pub const CLASSNAME: &'static str = "Identity";
    pub const FIELDNAME_ID: &'static str = "id";
    pub const FIELDNAME_REGION_ID: &'static str = "region_id";
    pub const FIELDNAME_WORLD_ID: &'static str = "world_id";
    pub const FIELDNAME_UNIVERSE_ID: &'static str = "universe_id";

    pub const FIELD_ID: Field = Field::new(Self::FIELDNAME_ID, FieldValueType::String);
    pub const FIELD_REGION_ID: Field = Field::new(Self::FIELDNAME_REGION_ID, FieldValueType::StringArray);
    pub const FIELD_WORLD_ID: Field = Field::new(Self::FIELDNAME_WORLD_ID, FieldValueType::String);
    pub const FIELD_UNIVERSE_ID: Field = Field::new(Self::FIELDNAME_UNIVERSE_ID, FieldValueType::String);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::ID => &Self::FIELD_ID,
            Self::RegionID => &Self::FIELD_REGION_ID,
            Self::WorldID => &Self::FIELD_WORLD_ID,
            Self::UniverseID => &Self::FIELD_UNIVERSE_ID
        }
    }
}

pub struct IdentityBuilder {
    builder_mode: BuilderMode,
    id: Option<ID>,
    region_id: Option<RegionID>,
    world_id: Option<WorldID>,
    universe_id: Option<UniverseID>
}

impl Builder for IdentityBuilder {
    type Type = Identity;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            id: None,
            region_id: None,
            world_id: None,
            universe_id: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator()
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn create(self) -> Result<Self::Type> {
        Ok(Identity {
            id: self.id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_ID})?,
            region_id: self.region_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_REGION_ID})?,
            world_id: self.world_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_WORLD_ID})?,
            universe_id: self.universe_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_UNIVERSE_ID})?
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        let mut fields_changed = Vec::new();

        if let Some(id) = self.id {
            original.id = id;
            fields_changed.push(IdentityField::ID.field());
        }
        if let Some(region_id) = self.region_id {
            original.region_id = region_id;
            fields_changed.push(IdentityField::RegionID.field());
        }
        if let Some(world_id) = self.world_id {
            original.world_id = world_id;
            fields_changed.push(IdentityField::WorldID.field());
        }
        if let Some(universe_id) = self.universe_id {
            original.universe_id = universe_id;
            fields_changed.push(IdentityField::UniverseID.field());
        }

        Ok(ModifyResult::new(fields_changed))
    }
}

impl Build for Identity {
    type BuilderType = IdentityBuilder;
}

