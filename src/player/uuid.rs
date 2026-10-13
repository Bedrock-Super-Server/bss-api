//! UUIDs, as Bedrock uses them to identify players.

/// The variant of a [`Uuid`], from its most significant bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// Reserved for NCS backward compatibility.
    Ncs,
    /// The variant of RFC 9562 (formerly RFC 4122), used by nearly every UUID.
    Rfc4122,
    /// Reserved for Microsoft backward compatibility.
    Microsoft,
    /// Reserved for future use.
    Future,
}

/// The version of a [`Uuid`], which says how it was generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// Version 1: a timestamp and a node id.
    TimeBased,
    /// Version 2: DCE security.
    DceSecurity,
    /// Version 3: an MD5 hash of a name.
    Md5,
    /// Version 4: random.
    Random,
    /// Version 5: a SHA-1 hash of a name.
    Sha1,
    /// Version 6: version 1 reordered to sort by time.
    SortableTime,
    /// Version 7: a Unix timestamp in milliseconds and random bits.
    UnixTime,
    /// Version 8: a custom layout.
    Custom,
    /// A version number no specification assigns.
    Unassigned(u8),
}

/**
A 128-bit universally unique identifier.

```
use bss_api::prelude::*;

let uuid = Uuid::parse("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
assert_eq!(uuid.version(), Version::TimeBased);
assert_eq!(uuid.variant(), Variant::Rfc4122);
assert_eq!(uuid.to_hyphenated(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
```
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    bytes: [u8; 16],
}

const GREGORIAN_OFFSET: i64 = 122_192_928_000_000_000;

impl Uuid {
    /// Parses a UUID written as 32 hexadecimal digits, with or without the four hyphens. Returns `None` for anything else.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        let hex: Vec<u8> = match text.len() {
            36 => {
                let bytes = text.as_bytes();
                if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
                    return None;
                }
                bytes
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !matches!(i, 8 | 13 | 18 | 23))
                    .map(|(_, c)| *c)
                    .collect()
            }
            32 => text.as_bytes().to_vec(),
            _ => return None,
        };

        let mut out = [0u8; 16];
        for (byte, pair) in out.iter_mut().zip(hex.as_chunks::<2>().0) {
            *byte = (nibble(pair[0])? << 4) | nibble(pair[1])?;
        }
        Some(Self { bytes: out })
    }

    /// Wraps 16 bytes, most significant first.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }

    /// Returns the 16 bytes, most significant first.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Returns the version number, from `0` to `15`.
    pub fn version_number(&self) -> u8 {
        self.bytes[6] >> 4
    }

    /// Returns the version.
    pub fn version(&self) -> Version {
        match self.version_number() {
            1 => Version::TimeBased,
            2 => Version::DceSecurity,
            3 => Version::Md5,
            4 => Version::Random,
            5 => Version::Sha1,
            6 => Version::SortableTime,
            7 => Version::UnixTime,
            8 => Version::Custom,
            other => Version::Unassigned(other),
        }
    }

    /// Returns the variant.
    pub fn variant(&self) -> Variant {
        let byte = self.bytes[8];
        if byte & 0x80 == 0 {
            Variant::Ncs
        } else if byte & 0x40 == 0 {
            Variant::Rfc4122
        } else if byte & 0x20 == 0 {
            Variant::Microsoft
        } else {
            Variant::Future
        }
    }

    /// Returns `true` for the nil UUID, all zeros.
    pub fn is_nil(&self) -> bool {
        self.bytes == [0u8; 16]
    }

    /// Returns `true` for the max UUID, all ones.
    pub fn is_max(&self) -> bool {
        self.bytes == [0xFFu8; 16]
    }

    /// Returns `true` for a UUID of the RFC variant with an assigned version.
    pub fn is_valid(&self) -> bool {
        self.variant() == Variant::Rfc4122 && !matches!(self.version(), Version::Unassigned(_))
    }

    /// Returns the time embedded in a version 1, 6 or 7 UUID, in milliseconds since the Unix epoch.
    pub fn timestamp_ms(&self) -> Option<i64> {
        let b = &self.bytes;
        match self.version() {
            /* Versions 1 and 6 count 100 ns intervals since 1582-10-15, the start
             * of the Gregorian calendar, in 60 bits: version 1 stores them low
             * part first, version 6 high part first so that UUIDs sort by time.
             * Version 7 stores Unix milliseconds directly in its first 48 bits. */
            Version::TimeBased => {
                let low = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as i64;
                let mid = u16::from_be_bytes([b[4], b[5]]) as i64;
                let high = (u16::from_be_bytes([b[6], b[7]]) & 0x0FFF) as i64;
                gregorian_to_unix_ms((high << 48) | (mid << 32) | low)
            }
            Version::SortableTime => {
                let high = u64::from_be_bytes([0, 0, b[0], b[1], b[2], b[3], b[4], b[5]]) as i64;
                let low = (u16::from_be_bytes([b[6], b[7]]) & 0x0FFF) as i64;
                gregorian_to_unix_ms((high << 12) | low)
            }
            Version::UnixTime => {
                Some(u64::from_be_bytes([0, 0, b[0], b[1], b[2], b[3], b[4], b[5]]) as i64)
            }
            _ => None,
        }
    }

    /// Returns the node id of a version 1 or 6 UUID.
    pub fn node(&self) -> Option<[u8; 6]> {
        matches!(self.version(), Version::TimeBased | Version::SortableTime).then(|| {
            let mut node = [0u8; 6];
            node.copy_from_slice(&self.bytes[10..16]);
            node
        })
    }

    /// Returns whether the node id of a version 1 or 6 UUID is random rather than a MAC address.
    pub fn node_is_random(&self) -> Option<bool> {
        self.node().map(|node| node[0] & 0x01 == 1)
    }

    /// Formats the UUID in lowercase, with hyphens.
    pub fn to_hyphenated(&self) -> String {
        let b = &self.bytes;
        let hex = |slice: &[u8]| {
            slice
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        };
        format!(
            "{}-{}-{}-{}-{}",
            hex(&b[0..4]),
            hex(&b[4..6]),
            hex(&b[6..8]),
            hex(&b[8..10]),
            hex(&b[10..16])
        )
    }
}

/// Returns `true` when `text` parses as a [`valid`](Uuid::is_valid) UUID.
pub fn is_valid(text: &str) -> bool {
    Uuid::parse(text).is_some_and(|uuid| uuid.is_valid())
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn gregorian_to_unix_ms(ticks: i64) -> Option<i64> {
    ticks
        .checked_sub(GREGORIAN_OFFSET)
        .map(|since_epoch| since_epoch / 10_000)
        .filter(|ms| *ms >= 0)
}
