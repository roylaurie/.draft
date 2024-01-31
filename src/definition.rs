pub mod standard;

use crate::{definition::standard::*, equation::*, id::*, index::Index, ACCT_SYSTEM_ID, ACCT_VERSION_ID};

pub trait AccountDefinitionTrait {
    fn id(&self) -> ID;
    fn name(&self) -> &str;
    fn equation_variable(&self) -> Equation;
    fn parent<'index>(&self, index: &'index Index) -> Option<&'index AccountDefinition>;
    fn is_standard(&self) -> bool;
}

#[derive(Debug)]
pub enum AccountDefinition {
    Standard(StandardAccountDefinition),
    Custom(CustomAccountDefinition)
}

impl AccountDefinitionTrait for AccountDefinition {
    fn name(&self) -> &str {
        match self {
            AccountDefinition::Standard(def) => def.name(),
            AccountDefinition::Custom(def) => def.name(),
        }
    }

    fn equation_variable(&self) -> Equation {
        match self {
            AccountDefinition::Standard(def) => def.equation_variable(),
            AccountDefinition::Custom(def) => def.equation_variable(),
        }
    }

    fn parent<'index>(&self, index: &'index Index) -> Option<&'index AccountDefinition> {
        match self {
            AccountDefinition::Standard(def) => def.parent(index),
            AccountDefinition::Custom(def) => def.parent(index),
        }
    }

    fn is_standard(&self) -> bool {
        match self {
            AccountDefinition::Standard(def) => def.is_standard(),
            AccountDefinition::Custom(def) => def.is_standard(),
        }

    }

    fn id(&self) -> ID {
        match self {
            AccountDefinition::Standard(def) => def.id(),
            AccountDefinition::Custom(def) => def.id(),
        }
    }
}

impl AccountDefinitionTrait for StandardAccounts {
    fn name(&self) -> &str {
        self.definition().name()
    }

    fn equation_variable(&self) -> Equation {
        self.definition().equation_variable() 
    }

    fn parent<'index>(&self, index: &'index Index) -> Option<&'index AccountDefinition> {
        self.definition().parent(index)
    }

    fn is_standard(&self) -> bool {
        true
    }

    fn id(&self) -> ID {
        self.definition().id()
    }
}

#[derive(Debug)]
pub struct StandardAccountDefinition {
    id: ID,
    name: &'static str,
    equation_variable: Equation,
    parent: Option<&'static AccountDefinition>,
}

impl ClassIdentityTrait for StandardAccountDefinition {
    fn class_identity(&self) -> ClassIdentity {
        Self::CLASS_IDENTITY
    }
}

impl StandardAccountDefinition {
    pub const CLASS_IDENTITY: ClassIdentity = ClassIdentity::StandardAccountDefinition; 

    pub const fn root(id_serial: u32, name: &'static str, equation_variable: Equation) -> AccountDefinition {
        AccountDefinition::Standard(Self {
            id: ID::v1_system(ACCT_SYSTEM_ID, ACCT_VERSION_ID, Self::CLASS_IDENTITY.into_class_id(), id_serial),
            name,
            equation_variable,
            parent: None,
        })
    }

    pub const fn sub(id_serial: u32, name: &'static str, parent: &'static AccountDefinition) -> AccountDefinition {
        let parent_def = if let AccountDefinition::Standard(standard_def) = parent {
            standard_def
        } else {
            panic!("Only AccountDefinition::Standard is allowed as parent");
        };

        AccountDefinition::Standard(Self {
            id: ID::v1_system(ACCT_SYSTEM_ID, ACCT_VERSION_ID, Self::CLASS_IDENTITY.into_class_id(), id_serial),
            name,
            equation_variable: parent_def.equation_variable_const(),
            parent: Some(parent),
        })
    }

}

impl AccountDefinitionTrait for StandardAccountDefinition {
    fn name(&self) -> &str {
        &self.name
    }

    fn equation_variable(&self) -> Equation {
        self.equation_variable
    }

    fn parent<'index>(&self, _index: &'index Index) -> Option<&'index AccountDefinition> {
        self.parent
    }

    fn is_standard(&self) -> bool {
        true
    }

    fn id(&self) -> ID {
        self.id
    }
}

impl StandardAccountDefinition {
    pub const fn equation_variable_const(&self) -> Equation {
        self.equation_variable
    }
}

#[derive(Debug)]
pub struct CustomAccountDefinition {
    id: ID,
    name: String,
    equation_variable: Equation,
    parent_id: Option<ID>,
}

impl AccountDefinitionTrait for CustomAccountDefinition {
    fn name(&self) -> &str {
        &self.name
    }

    fn equation_variable(&self) -> Equation {
        self.equation_variable
    }

    fn parent<'index>(&self, index: &'index Index) -> Option<&'index AccountDefinition> {
        if let Some(id) = self.parent_id {
            index.definition(id)
        } else {
            None
        }
    }

    fn is_standard(&self) -> bool {
        false
    }

    fn id(&self) -> ID {
        self.id
    }
}

impl ClassIdentityTrait for CustomAccountDefinition {
    fn class_identity(&self) -> ClassIdentity {
        Self::CLASS_IDENTITY
    }
}

impl CustomAccountDefinition {
    pub const CLASS_IDENTITY: ClassIdentity = ClassIdentity::CustomAccountDefinition;

    pub fn new(id: ID, name: String, equation_variable: Equation, parent_id: ID) -> Self {
        Self {
            id,
            name,
            equation_variable,
            parent_id: Some(parent_id) 
        }
    }

    pub fn new_root(id: ID, equation_variable: Equation, name: String) -> Self {
        Self {
            id,
            name,
            equation_variable,
            parent_id: None
        }
    }
}

impl AsRef<AccountDefinition> for &AccountDefinition {
    fn as_ref(&self) -> &AccountDefinition {
        self
    }
}
