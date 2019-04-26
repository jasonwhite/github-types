// Copyright (c) 2019 Jason White
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::fmt;
use std::ops;

use hex::{FromHex, FromHexError, ToHex};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{self, Serialize, Serializer};

/// A Git object ID (i.e., a SHA1).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Oid([u8; 20]);

impl Oid {
    pub fn from_hex(s: &str) -> Result<Self, ()> {
        Ok(Oid(<[u8; 20]>::from_hex(s).map_err(|_| ())?))
    }

    /// The empty tree sha `4b825dc642cb6eb9a060e54bf8d69288fbee4904`.
    ///
    /// This can be computed manually with `git hash-object -t tree /dev/null`.
    pub const EMPTY_TREE: Oid = Oid([
        0x4b, 0x82, 0x5d, 0xc6, 0x42, 0xcb, 0x6e, 0xb9, 0xa0, 0x60, 0xe5, 0x4b,
        0xf8, 0xd6, 0x92, 0x88, 0xfb, 0xee, 0x49, 0x04,
    ]);

    /// A sha of all zeros. Usually used to indicate that a branch is either
    /// created or deleted.
    pub const ZERO: Oid = Oid([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
}

impl fmt::UpperHex for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_hex_upper(f)
    }
}

impl fmt::LowerHex for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_hex(f)
    }
}

impl fmt::Display for Oid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::LowerHex>::fmt(self, f)
    }
}

impl fmt::Debug for Oid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl ops::Deref for Oid {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            // Serialize as a hex string.
            let mut hex = String::new();
            self.0
                .as_ref()
                .write_hex(&mut hex)
                .map_err(ser::Error::custom)?;
            serializer.serialize_str(&hex)
        } else {
            // Serialize as a byte array with known length.
            serializer.serialize_bytes(self.0.as_ref())
        }
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OidVisitor;

        impl<'de> Visitor<'de> for OidVisitor {
            type Value = Oid;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "hex string or 20 bytes")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let v = <[u8; 20]>::from_hex(v).map_err(|e| match e {
                    FromHexError::InvalidHexCharacter { c, .. } => {
                        E::invalid_value(
                            de::Unexpected::Char(c),
                            &"string with only hexadecimal characters",
                        )
                    }
                    FromHexError::InvalidStringLength => E::invalid_length(
                        v.len(),
                        &"hex string with a valid length",
                    ),
                    FromHexError::OddLength => E::invalid_length(
                        v.len(),
                        &"hex string with an even length",
                    ),
                })?;

                Ok(Oid(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.len() != 20 {
                    return Err(E::invalid_length(v.len(), &"20 bytes"));
                }

                let mut inner = <[u8; 20]>::default();
                inner.copy_from_slice(v);

                Ok(Oid(inner))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(OidVisitor)
        } else {
            deserializer.deserialize_bytes(OidVisitor)
        }
    }
}
