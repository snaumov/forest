// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::VRFProof;
use encoding::tuple::*;

/// Proofs generated by a miner which determines the reward they earn.
/// This is generated from hashing a partial ticket and using the hash to generate a value.
#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Default, Ord, Serialize_tuple, Deserialize_tuple,
)]
pub struct ElectionProof {
    pub win_count: i64,
    pub vrfproof: VRFProof,
}

#[cfg(feature = "json")]
pub mod json {
    use super::*;
    use crate::vrf;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Wrapper for serializing and deserializing a ElectionProof from JSON.
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct ElectionProofJson(#[serde(with = "self")] pub ElectionProof);

    /// Wrapper for serializing a ElectionProof reference to JSON.
    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct ElectionProofJsonRef<'a>(#[serde(with = "self")] pub &'a ElectionProof);

    pub fn serialize<S>(m: &ElectionProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct ElectionProofSer<'a> {
            #[serde(rename = "VRFProof", with = "vrf::json")]
            vrfproof: &'a VRFProof,
            win_count: i64,
        }
        ElectionProofSer {
            vrfproof: &m.vrfproof,
            win_count: m.win_count,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ElectionProof, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct ElectionProofDe {
            #[serde(rename = "VRFProof", with = "vrf::json")]
            vrfproof: VRFProof,
            win_count: i64,
        }
        let ElectionProofDe {
            vrfproof,
            win_count,
        } = Deserialize::deserialize(deserializer)?;
        Ok(ElectionProof {
            vrfproof,
            win_count,
        })
    }

    pub mod opt {
        use super::{ElectionProof, ElectionProofJson, ElectionProofJsonRef};
        use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S>(v: &Option<ElectionProof>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            v.as_ref()
                .map(|s| ElectionProofJsonRef(s))
                .serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ElectionProof>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<ElectionProofJson> = Deserialize::deserialize(deserializer)?;
            Ok(s.map(|v| v.0))
        }
    }
}
