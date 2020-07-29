// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::beacon_entries::BeaconEntry;
use super::drand_api::api::PublicRandRequest;
use super::drand_api::api_grpc::PublicClient;
use super::drand_api::common::GroupRequest;
use super::group::Group;

use async_trait::async_trait;
use bls_signatures::{PublicKey, Serialize, Signature};
use byteorder::{BigEndian, WriteBytesExt};
use clock::ChainEpoch;
use grpc::ClientStub;
use grpc::RequestOptions;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use sha2::Digest;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::sync::Arc;
use tls_api_rustls::TlsConnector;

/// Coeffiencients of the publicly available Drand keys.
/// This is shared by all participants on the Drand network.
#[derive(Clone, Debug, SerdeSerialize, SerdeDeserialize)]
pub struct DistPublic {
    pub coefficients: [Vec<u8>; 4],
}

impl DistPublic {
    pub fn key(&self) -> PublicKey {
        PublicKey::from_bytes(&self.coefficients[0]).unwrap()
    }
}

#[async_trait]
pub trait Beacon
where
    Self: Sized,
{
    /// Verify a new beacon entry against the most recent one before it.
    fn verify_entry(
        &mut self,
        curr: &BeaconEntry,
        prev: &BeaconEntry,
    ) -> Result<bool, Box<dyn error::Error>>;

    /// Returns a BeaconEntry given a round. It fetches the BeaconEntry from a Drand node over GRPC
    /// In the future, we will cache values, and support streaming.
    async fn entry(&self, round: u64) -> Result<BeaconEntry, Box<dyn error::Error>>;

    fn max_beacon_round_for_epoch(&self, fil_epoch: ChainEpoch) -> u64;
}

pub struct DrandBeacon {
    client: PublicClient,
    pub_key: DistPublic,
    interval: u64,
    drand_gen_time: u64,
    fil_gen_time: u64,
    fil_round_time: u64,

    // Keeps track of computed beacon entries.
    local_cache: HashMap<u64, BeaconEntry>,
}

impl DrandBeacon {
    /// Construct a new DrandBeacon.
    pub async fn new(
        pub_key: DistPublic,
        genesis_ts: u64,
        interval: u64,
    ) -> Result<Self, Box<dyn error::Error>> {
        if genesis_ts == 0 {
            panic!("Genesis timestamp cannot be 0")
        }
        // construct grpc client
        // TODO: Allow to randomize between different drand servers
        let client = grpc::ClientBuilder::new("nicolas.drand.fil-test.net", 443)
            .tls::<TlsConnector>()
            .build()
            .unwrap();
        let client = PublicClient::with_client(Arc::new(client));

        // get nodes in group
        let req = GroupRequest::new();
        let group_resp = client
            .group(RequestOptions::new(), req)
            .drop_metadata()
            .await?;
        let group: Group = Group::try_from(group_resp)?;
        // TODO: Compare pubkeys with one in config

        Ok(Self {
            pub_key,
            client,
            interval: group.period as u64,
            drand_gen_time: group.genesis_time,
            fil_round_time: interval,
            fil_gen_time: genesis_ts,
            local_cache: HashMap::new(),
        })
    }
}
/// This struct allows you to talk to a Drand node over GRPC.
/// Use this to source randomness and to verify Drand beacon entries.
#[async_trait]
impl Beacon for DrandBeacon {
    fn verify_entry(
        &mut self,
        curr: &BeaconEntry,
        prev: &BeaconEntry,
    ) -> Result<bool, Box<dyn error::Error>> {
        // TODO: Handle Genesis better
        if prev.round() == 0 {
            return Ok(true);
        }

        // Hash the messages
        let mut msg: Vec<u8> = Vec::with_capacity(104);
        msg.extend_from_slice(prev.data());
        msg.write_u64::<BigEndian>(curr.round())?;
        // H(prev sig | curr_round)
        let digest = sha2::Sha256::digest(&msg);
        // Hash to G2
        let digest = bls_signatures::hash(&digest);
        // Signature
        let sig = Signature::from_bytes(curr.data())?;
        let sig_match = bls_signatures::verify(&sig, &[digest], &[self.pub_key.key()]);

        // Cache the result
        if sig_match && !self.local_cache.contains_key(&curr.round()) {
            self.local_cache.insert(curr.round(), curr.clone());
        }
        Ok(sig_match)
    }

    async fn entry(&self, round: u64) -> Result<BeaconEntry, Box<dyn error::Error>> {
        match self.local_cache.get(&round) {
            Some(cached_entry) => Ok(cached_entry.clone()),
            None => {
                let mut req = PublicRandRequest::new();
                req.round = round;
                let resp = self
                    .client
                    .public_rand(grpc::RequestOptions::new(), req)
                    .drop_metadata()
                    .await?;

                Ok(BeaconEntry::new(resp.round, resp.signature))
            }
        }
    }

    fn max_beacon_round_for_epoch(&self, fil_epoch: ChainEpoch) -> u64 {
        let latest_ts =
            fil_epoch as u64 * self.fil_round_time + self.fil_gen_time - self.fil_round_time;
        // TODO: self.interval has to be converted to seconds. Dont know what it is right now
        (latest_ts - self.drand_gen_time) / self.interval
    }
}
