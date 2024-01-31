pub type UID = u128;
// The system or API registered with a network of Authorities.
pub type SystemID = u16;
// The version of the system. Unique for the system.
pub type VersionID = u8;
/// The object class. Unique for a System.
pub type ClassID = u16;
/// The service provider that issues IDs for an independent system.  
/// ID of 0 is used as "Local" (not GUID). Conversley, non-zero IDs indiciate a (registered) Globally Unique ID system.
pub type AuthorityID = u8;
/// The client organization or organizational unit that this object belongs to. Unique for an Authority.
/// ID of 0 can be used as "Local", but only for a "Local" authority.
pub type DomainID = u32;
/// Unique to a domain. Used as a shard, page, etc. Zero indexed.
pub type SegmentID = u16; 
/// The object ID. Unique for its Segment. ID of 0 is invalid. ~4 billion max objects per class per org (per shard).
pub type SerialID = u32;

const UID_BITS:       usize = std::mem::size_of::<UID>()         * 8;
const SYSTEM_BITS:    usize = std::mem::size_of::<SystemID>()    * 8;
const VERSION_BITS:   usize = std::mem::size_of::<VersionID>()   * 8;
const CLASS_BITS:     usize = std::mem::size_of::<ClassID>()     * 8;
const AUTHORITY_BITS: usize = std::mem::size_of::<AuthorityID>() * 8;
const DOMAIN_BITS:    usize = std::mem::size_of::<DomainID>()    * 8;
const SEGMENT_BITS:   usize = std::mem::size_of::<SegmentID>()   * 8;
const SERIAL_BITS:    usize = std::mem::size_of::<SerialID>()    * 8;

const SYSTEM_SHIFT:    usize = UID_BITS        - SYSTEM_BITS;
const VERSION_SHIFT:   usize = SYSTEM_SHIFT    - VERSION_BITS;
const CLASS_SHIFT:     usize = VERSION_SHIFT   - CLASS_BITS;
const AUTHORITY_SHIFT: usize = CLASS_SHIFT     - AUTHORITY_BITS;
const DOMAIN_SHIFT:    usize = AUTHORITY_SHIFT - DOMAIN_BITS;
const SEGMENT_SHIFT:   usize = DOMAIN_SHIFT    - SEGMENT_BITS;
const SERIAL_SHIFT:    usize = SEGMENT_SHIFT   - SERIAL_BITS;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ID {
    pub system:    SystemID,
    pub version:   VersionID,
    pub class:     ClassID,
    pub authority: AuthorityID,
    pub domain:    DomainID,
    pub segment:   SegmentID,
    pub serial:    SerialID,
}

impl ID {
    pub const ZERO: Self = Self::v1(0,0,0,0,0,0,0);
    pub const AUTHORITY_LOCAL: AuthorityID = 0;
    pub const DOMAIN_LOCAL: DomainID = 0;

    pub const fn v1(
        system: SystemID,
        version: VersionID,
        class: ClassID,
        authority: AuthorityID,
        domain: DomainID,
        segment: SegmentID,
        serial: SerialID
    ) -> Self {
        Self {
            system,
            version,
            class,
            authority,
            domain,
            segment,
            serial 
        }
    }

    /// Hard-coded values
    pub const fn v1_system(system: SystemID, version: VersionID, class: ClassID, serial: SerialID) -> Self {
        Self {
            system,
            version,
            class,
            authority: Self::AUTHORITY_LOCAL,
            domain: Self::DOMAIN_LOCAL,
            segment: 0,
            serial 
        }
    }

    /// The authority and domain are local
    pub const fn v1_localhost(system: SystemID, version: VersionID, class: ClassID, segment: SegmentID, serial: SerialID) -> Self {
        Self {
            system,
            version,
            class,
            authority: Self::AUTHORITY_LOCAL,
            domain: Self::DOMAIN_LOCAL,
            segment,
            serial 
        }
    }

    pub const fn valid(&self) -> bool {
        self.class != 0 && self.serial != 0
        && ( self.authority == 0 || (self.authority != 0 && self.domain != 0) )
    }

    pub const fn from_uid(value: UID) -> Self {
        Self {
            system:    (value >> SYSTEM_SHIFT)    as SystemID,
            version:   (value >> VERSION_SHIFT)   as VersionID,
            class:     (value >> CLASS_SHIFT)     as ClassID,
            authority: (value >> AUTHORITY_SHIFT) as AuthorityID,
            domain:    (value >> DOMAIN_SHIFT)    as DomainID,
            segment:   (value >> SEGMENT_SHIFT)   as SegmentID,
            serial:    (value >> SERIAL_SHIFT)    as SerialID 
        }
    }

    pub const fn into_uid(self) -> UID {
        0
        | ((self.system    as UID) << SYSTEM_SHIFT)
        | ((self.version   as UID) << VERSION_SHIFT)
        | ((self.class     as UID) << CLASS_SHIFT)
        | ((self.authority as UID) << AUTHORITY_SHIFT)
        | ((self.domain    as UID) << DOMAIN_SHIFT)
        | ((self.segment   as UID) << SEGMENT_SHIFT)
        | ((self.serial    as UID) << SERIAL_SHIFT)
    }

    /// Does this represent a GUID (true) or not.
    pub const fn is_global(&self) -> bool {
        self.authority != Self::AUTHORITY_LOCAL
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
pub trait ClassIdentityTrait {
    fn class_identity(&self) -> ClassIdentity;
}

/// Unit-only enum representing a harcoded set of ClassIDs reserved for known system-wide types.  
/// Implements `Into<ClassID>` and `From<ClassID>` for conversion.
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
    fn test_v1_system_min_mid_max() {
        let expected: [(SystemID, VersionID, ClassID, SerialID);4] = [
            (SystemID::MIN, VersionID::MIN, ClassID::MIN, SerialID::MIN),
            (SystemID::MIN + 1, VersionID::MIN + 1, ClassID::MIN + 1, SerialID::MIN + 1),
            (SystemID::MAX / 2, VersionID::MAX / 2, ClassID::MAX / 2, SerialID::MAX / 2),
            (SystemID::MAX, VersionID::MAX, ClassID::MAX, SerialID::MAX),
        ];

        for (system_id, version_id, class_id, serial_id) in expected {
            let id = ID::v1_system(system_id, version_id, class_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(ID::AUTHORITY_LOCAL, id.authority);
            assert_eq!(ID::DOMAIN_LOCAL, id.domain);
            assert_eq!(0, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

    #[test]
    fn test_v1_localhost_min_mid_max() {
        let expected: [(SystemID, VersionID, ClassID, SegmentID, SerialID);4] = [
            (SystemID::MIN, VersionID::MIN, ClassID::MIN, SegmentID::MIN, SerialID::MIN),
            (SystemID::MIN + 1, VersionID::MIN + 1, ClassID::MIN + 1, SegmentID::MIN, SerialID::MIN + 1),
            (SystemID::MAX / 2, VersionID::MAX / 2, ClassID::MAX / 2, SegmentID::MIN, SerialID::MAX / 2),
            (SystemID::MAX, VersionID::MAX, ClassID::MAX, SegmentID::MIN, SerialID::MAX),
        ];

        for (system_id, version_id, class_id, segment_id, serial_id) in expected {
            let id = ID::v1_localhost(system_id, version_id, class_id, segment_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(ID::AUTHORITY_LOCAL, id.authority);
            assert_eq!(ID::DOMAIN_LOCAL, id.domain);
            assert_eq!(segment_id, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

    #[test]
    fn test_v1_min_mid_max() {
        let expected: [(SystemID, VersionID, ClassID, AuthorityID, DomainID, SegmentID, SerialID);4] = [
            (SystemID::MIN, VersionID::MIN, ClassID::MIN, AuthorityID::MIN, DomainID::MIN, SegmentID::MIN, SerialID::MIN),
            (SystemID::MIN + 1, VersionID::MIN + 1, ClassID::MIN + 1, AuthorityID::MIN + 1, DomainID::MIN + 1, SegmentID::MIN + 1, SerialID::MIN + 1),
            (SystemID::MAX / 2, VersionID::MAX / 2, ClassID::MAX / 2, AuthorityID::MAX / 2, DomainID::MAX / 2, SegmentID::MAX, SerialID::MAX / 2),
            (SystemID::MAX, VersionID::MAX, ClassID::MAX, AuthorityID::MAX, DomainID::MAX, SegmentID::MAX, SerialID::MAX),
        ];

        for (system_id, version_id, class_id, authority_id, domain_id, segment_id, serial_id) in expected {
            let id = ID::v1(system_id, version_id, class_id, authority_id, domain_id, segment_id, serial_id);
            let uid: UID = id.into();
            let id = ID::from(uid);

            assert_eq!(class_id, id.class);
            assert_eq!(authority_id, id.authority);
            assert_eq!(domain_id, id.domain);
            assert_eq!(segment_id, id.segment);
            assert_eq!(serial_id, id.serial);
        }
    }

}
