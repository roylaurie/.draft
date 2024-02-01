pub type UID = u128;
/// The publsher, system, suite, product, or API.  
/// For organizations, this should typically represent the entire org or a subdivision of it.
/// ID of 0 can be used as "Local" (unregistered).
pub type SystemID = u32;
/// The object class. Unique for a System.
pub type ClassID = u16;
/// The client organization or organizational unit that this object belongs to. Unique for an Authority.
/// ID of 0 can be used as "Self".
pub type DomainID = u32;
/// Unique to a domain. Used as a shard, page, etc. Zero indexed.
/// ~65k possible shards.
pub type SegmentID = u16; 
/// The object ID. Unique for its Segment. ID of 0 is invalid.
/// ~4 billion possible objects per class per org (per shard).
pub type SerialID = u32;

const UID_BITS:       usize = std::mem::size_of::<UID>()         * 8;
const SYSTEM_BITS:    usize = std::mem::size_of::<SystemID>()    * 8;
const CLASS_BITS:     usize = std::mem::size_of::<ClassID>()     * 8;
const DOMAIN_BITS:    usize = std::mem::size_of::<DomainID>()    * 8;
const SEGMENT_BITS:   usize = std::mem::size_of::<SegmentID>()   * 8;
const SERIAL_BITS:    usize = std::mem::size_of::<SerialID>()    * 8;

const SYSTEM_SHIFT:    usize = UID_BITS        - SYSTEM_BITS;
const CLASS_SHIFT:     usize = SYSTEM_SHIFT    - CLASS_BITS;
const DOMAIN_SHIFT:    usize = CLASS_SHIFT     - DOMAIN_BITS;
const SEGMENT_SHIFT:   usize = DOMAIN_SHIFT    - SEGMENT_BITS;
const SERIAL_SHIFT:    usize = SEGMENT_SHIFT   - SERIAL_BITS;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ID {
    pub system:    SystemID,
    pub class:     ClassID,
    pub domain:    DomainID,
    pub segment:   SegmentID,
    pub serial:    SerialID,
}

impl ID {
    pub const ZERO: Self = Self::v1(0,0,0,0,0);
    pub const SYSTEM_LOCAL: SystemID = 0;
    pub const DOMAIN_LOCAL: DomainID = 0;
    pub const SEGMENT_PRIMARY: SegmentID = 0;

    pub const fn v1(
        system: SystemID,
        class: ClassID,
        domain: DomainID,
        segment: SegmentID,
        serial: SerialID
    ) -> Self {
        Self {
            system,
            class,
            domain,
            segment,
            serial 
        }
    }

    /// Hard-coded values
    pub const fn v1_system(system: SystemID, class: ClassID, serial: SerialID) -> Self {
        Self {
            system,
            class,
            domain: Self::DOMAIN_LOCAL,
            segment: Self::SEGMENT_PRIMARY,
            serial 
        }
    }

    /// Domain and system are local 
    pub const fn v1_local(class: ClassID, segment: SegmentID, serial: SerialID) -> Self {
        Self {
            system: Self::SYSTEM_LOCAL,
            class,
            domain: Self::DOMAIN_LOCAL,
            segment,
            serial 
        }
    }

    pub const fn valid(&self) -> bool {
        self.class != 0 && self.serial != 0
    }

    pub const fn from_uid(value: UID) -> Self {
        Self {
            system:    (value >> SYSTEM_SHIFT)    as SystemID,
            class:     (value >> CLASS_SHIFT)     as ClassID,
            domain:    (value >> DOMAIN_SHIFT)    as DomainID,
            segment:   (value >> SEGMENT_SHIFT)   as SegmentID,
            serial:    (value >> SERIAL_SHIFT)    as SerialID 
        }
    }

    pub const fn into_uid(self) -> UID {
        0
        | ((self.system    as UID) << SYSTEM_SHIFT)
        | ((self.class     as UID) << CLASS_SHIFT)
        | ((self.domain    as UID) << DOMAIN_SHIFT)
        | ((self.segment   as UID) << SEGMENT_SHIFT)
        | ((self.serial    as UID) << SERIAL_SHIFT)
    }
}

impl Into<UID> for ID {
    fn into(self) -> UID {
        self.into_uid()
    }
}

impl From<UID> for ID {
    fn from(value: UID) -> Self {
        Self::from_uid(value)
    }
}

/// Associates a type with a const ClassIdentity -> ClassID 
pub trait ClassIdentity<E> {
    fn class_identity(&self) -> E;
}

/// Unit-only enum representing a harcoded set of ClassIDs reserved for known system-wide types.  
/// Implements `Into<ClassID>` and `From<ClassID>` for conversion.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountClassIdentity {
    // These descriminants are used in serialization and must never change once reserved, thus the explicit assignment.
    Unknown = 0,
    StandardAccountDefinition = 1,
    CustomAccountDefinition = 2,
}

impl From<ClassID> for AccountClassIdentity {
    fn from(value: ClassID) -> Self {
        Self::from_class_id(value)
    }
}

impl Into<ClassID> for AccountClassIdentity {
    fn into(self) -> ClassID {
        self.into_class_id() 
    }
}

impl AccountClassIdentity  {
    pub const fn from_class_id(value: ClassID) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::StandardAccountDefinition,
            2 => Self::CustomAccountDefinition,
            _ => Self::Unknown
        }
    }

    pub const fn into_class_id(self) -> ClassID {
        self as ClassID
    }
}

pub mod registered {
    pub enum RegisteredSystemID {
        Local = 0,
        Asmov = 1,
        AsmovGames = 2,
    }
}

pub mod asmov_id {
    #[allow(non_camel_case_types)]
    pub enum AsmovClassID {
        Invalid = 0,
        // Asmov Account (reserve 1-100)
        Account_CustomAccountDefinition = 1,
        Account_StandardDefinition      = 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_system_min_mid_max() {
        let expected: [(SystemID, ClassID, SerialID);4] = [
            (SystemID::MIN, ClassID::MIN, SerialID::MIN),
            (SystemID::MIN + 1, ClassID::MIN + 1, SerialID::MIN + 1),
            (SystemID::MAX / 2, ClassID::MAX / 2, SerialID::MAX / 2),
            (SystemID::MAX, ClassID::MAX, SerialID::MAX),
        ];

        for (system_id, class_id, serial_id) in expected {
            let id = ID::v1_system(system_id, class_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(ID::DOMAIN_LOCAL, id.domain);
            assert_eq!(0, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

    #[test]
    fn test_v1_local_min_mid_max() {
        let expected: [(ClassID, SegmentID, SerialID);4] = [
            (ClassID::MIN, SegmentID::MIN, SerialID::MIN),
            (ClassID::MIN + 1, SegmentID::MIN, SerialID::MIN + 1),
            (ClassID::MAX / 2, SegmentID::MIN, SerialID::MAX / 2),
            (ClassID::MAX, SegmentID::MIN, SerialID::MAX),
        ];

        for (class_id, segment_id, serial_id) in expected {
            let id = ID::v1_local(class_id, segment_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(ID::DOMAIN_LOCAL, id.domain);
            assert_eq!(segment_id, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

    #[test]
    fn test_v1_min_mid_max() {
        let expected: [(SystemID, ClassID, DomainID, SegmentID, SerialID);4] = [
            (SystemID::MIN, ClassID::MIN, DomainID::MIN, SegmentID::MIN, SerialID::MIN),
            (SystemID::MIN + 1, ClassID::MIN + 1, DomainID::MIN + 1, SegmentID::MIN + 1, SerialID::MIN + 1),
            (SystemID::MAX / 2, ClassID::MAX / 2, DomainID::MAX / 2, SegmentID::MAX, SerialID::MAX / 2),
            (SystemID::MAX, ClassID::MAX, DomainID::MAX, SegmentID::MAX, SerialID::MAX),
        ];

        for (system_id, class_id, domain_id, segment_id, serial_id) in expected {
            let id = ID::v1(system_id, class_id, domain_id, segment_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(domain_id, id.domain);
            assert_eq!(segment_id, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

}
