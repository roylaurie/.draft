use crate::model::builder::*;

/// All descriptive information about and object that can be observed by a player.
/// See also its corresponding trait: `Descriptive`
#[derive(Debug)]
pub struct Descriptor {
    /// The title
    name: String,
    /// Any term that might be used to reference this
    keywords: Vec<String>,
    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    key: Option<String>,
    /// A one-liner summary. If `description` is not available, this should be used instead.
    short_description: Option<String>,
    /// A detailed and narrative description.
    description: Option<String>,
}

/// The trait that provides standard immutable access to a `Descriptor` struct
pub trait Descriptive {
    /// Fetch the `Descriptor` struct for this object
    fn descriptor(&self) -> &Descriptor;

    /// The title
    fn name(&self) -> &str {
        &self.descriptor().name
    }

    /// Any term that might be used to reference this
    fn keywords(&self) -> &Vec<String> {
        &self.descriptor().keywords
    }

    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    fn key(&self) -> Option<&String> {
        self.descriptor().key.as_ref()
    }

    /// A one-liner summary. If `description` is not available, this will be used instead.
    fn short_description(&self) -> Option<&String> {
        self.descriptor().short_description.as_ref()
    }

    /// A detailed and narrative description. If this doesn't exist, `short_description` will be used instead. 
    fn description(&self) -> Option<&String> {
        self.descriptor().description.as_ref()
            .or_else(|| self.short_description())
    }
}

pub trait DescriptiveMut: Descriptive {
    fn descriptor_mut(&mut self) -> &mut Descriptor;
}

pub struct DescriptorBuilder<'original> {
    original: Option<&'original mut Descriptor>,
    name: Option<String>,
    keywords: Option<Vec<String>>,
    key: Option<String>,
    short_description: Option<String>,
    description: Option<String>
}

impl<'original> Builder<'original> for DescriptorBuilder<'original> {
    type Type = Descriptor;

    fn new() -> Self {
        Self {
            original: None,
            name: None,
            keywords: None,
            key: None,
            short_description: None,
            description: None
        }
    }

    fn editor(original: &'original mut Descriptor) -> Self {
        let mut s = Self::new();
        s.original = Some(original);
        s
    }

    fn build(self) -> Descriptor {
        Descriptor {
            name: self.name.expect("Name not set"),
            keywords: self.keywords.unwrap_or_else(|| Vec::new()),
            key: self.key,
            short_description: self.short_description,
            description: self.description
        }
    }

    fn edit(mut self, component_fields_changed: Option<Vec<Field>>) -> Result<Vec<Field>, ()> {
        let original = self.original.unwrap();
        let mut result = Vec::new();
        if let Some(name) = self.name {
            original.name = name;
            result.push(DescriptorField::name.field());
        }
        if self.description.is_some() {
            original.description = self.description;
            result.push(DescriptorField::description.field());
        }

        Ok(result)
    }

    fn set(&mut self, field: &str, raw_value: String) -> Result<(), ()> {
        match field {
            DescriptorBuilder::FIELD_NAME => { self.name(raw_value); },
            DescriptorBuilder::FIELD_DESCRIPTION => { self.description(raw_value); },
            _ => return Err(())
        }

        Ok(())
    }
}

impl DescriptorBuilder<'_> {
    pub const FIELD_NAME: &'static str = "name";
    pub const FIELD_DESCRIPTION: &'static str = "description";

    pub fn key(&mut self, key: String) -> &mut Self {
        self.key = Some(key);
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
}

impl<'original> Descriptor {
    pub fn builder() -> DescriptorBuilder<'original> {
        DescriptorBuilder::new()
    }

    pub fn editor(&'original mut self) -> DescriptorBuilder<'original> {
        DescriptorBuilder::editor(self)
    }
}

impl Descriptive for Descriptor {
    fn descriptor(&self) -> &Descriptor {
        &self
    }
}

#[allow(non_camel_case_types)]
pub enum DescriptorField {
    name,
    keywords,
    key,
    short_description,
    description
}

impl DescriptorField {
    pub const fn field(&self) -> Field {
        match self {
            Self::name => Field {
                name: DescriptorBuilder::FIELD_NAME,
                value_type: FieldValueType::String,
            },
            Self::keywords => Field {
                name: "keywords",
                value_type: FieldValueType::StringArray
            },
            Self::key => Field {
                name: "key",
                value_type: FieldValueType::String
            },
            Self::short_description => Field {
                name: "short_description",
                value_type: FieldValueType::String
            },
            Self::description => Field {
                name: DescriptorBuilder::FIELD_DESCRIPTION,
                value_type: FieldValueType::String
            }
        }
    }
}

