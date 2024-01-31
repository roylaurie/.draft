pub type DomainID = u32;
pub type ClassID  = u8;
pub type SerialID = u32;
pub type uID      = u128;

const DOMAIN_BITS: usize = std::mem::size_of::<DomainID>() * 8;
const CLASS_BITS:  usize = std::mem::size_of::<ClassID>()  * 8;
const SERIAL_BITS: usize = std::mem::size_of::<SerialID>() * 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ID {
    pub domain: DomainID,
    pub class: ClassID,
    pub serial: SerialID,
}

impl ID {
    pub const ZERO: Self = Self::new(0,0,0);
    pub const fn new(domain: DomainID, class: ClassID, serial: SerialID) -> Self {
        Self {
            domain,
            class,
            serial 
        }
    }

    pub fn valid(&self) -> bool {
        self.domain != 0 && self.class != 0 && self.serial != 0
    }
}

impl Into<uID> for ID {
    fn into(self) -> uID {
        0
        | ((self.domain as u128) << DOMAIN_BITS)
        | ((self.class as u128) << CLASS_BITS)
        | ((self.serial as u128) << SERIAL_BITS)
    }
}

impl From<uID> for ID {
    fn from(value: uID) -> Self {
        Self {
            domain: (value >> DOMAIN_BITS) as u32,
            class: (value >> CLASS_BITS) as u8,
            serial: (value >> SERIAL_BITS) as u32
        }
    }
}

/// Associates a type with a const ClassIdentity -> ClassID 
pub trait ClassIdentityTrait {
    fn class_identity(&self) -> ClassIdentity;
}

/// Unit-only enum representing a harcoded set of ClassIDs reserved for known data-types.  
/// Proves `Into<ClassID>` and `From<ClassID>` for conversion.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClassIdentity {
    // These descriminants are used in serialization and must never change once reserved, thus the explicit assignment.
    Unknown = 0,
    StandardAccountDefinition = 1,
    CustomAccountDefinition = 2,
}

impl From<ClassID> for ClassIdentity {
    fn from(value: ClassID) -> Self {
        Self::from_class_id(value)
    }
}

impl Into<ClassID> for ClassIdentity {
    fn into(self) -> ClassID {
        self.into_class_id() 
    }
}

impl ClassIdentity {
    pub const fn from_class_id(value: ClassID) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::StandardAccountDefinition,
            2 => Self::CustomAccountDefinition,
            _ => Self::Unknown
        }
    }

    pub const fn into_class_id(self) -> ClassID {
        match self {
            ClassIdentity::Unknown => 0,
            ClassIdentity::StandardAccountDefinition => 1,
            ClassIdentity::CustomAccountDefinition => 2,
        } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_mid_max() {
        let expected: [(DomainID, ClassID, SerialID);4] = [
            (DomainID::MIN, ClassID::MIN, SerialID::MIN),
            (DomainID::MIN + 1, ClassID::MIN + 1, SerialID::MIN + 1),
            (DomainID::MAX / 2, ClassID::MAX / 2, SerialID::MAX / 2),
            (DomainID::MAX, ClassID::MAX, SerialID::MAX),
        ];

        for (domain_id, class_id, serial_id) in expected {
            let id = ID::new(domain_id, class_id, serial_id);
            let uid: uID = id.into();
            let id = ID::from(uid);

            assert_eq!(domain_id, id.domain);
            assert_eq!(class_id, id.class);
            assert_eq!(serial_id, id.serial);
        }
    }
}
