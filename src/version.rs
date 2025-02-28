// Copyright 2017-2022 int08h LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Display, Formatter};

/// Version of the Roughtime protocol
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Version {
    /// Original Google version from https://roughtime.googlesource.com/roughtime/+/HEAD/PROTOCOL.md
    Classic,

    /// IETF standardized version
    Rfc,
}

// RFC version 1
const VERSION_1: &'static [u8] = &[0x01, 0x00, 0x00, 0x00];

impl Version {
    /// On-the-wire representation of the version value
    pub fn wire_bytes(self) -> &'static [u8] {
        match self {
            Version::Classic => unreachable!("invalid, no version bytes for Classic"),
            Version::Rfc => VERSION_1,
        }
    }

    /// A short (non-canonical) string representation of the `Version`
    pub fn to_string(&self) -> String {
        match self {
            Version::Classic => String::from("Classic"),
            Version::Rfc => String::from("Rfc"),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
