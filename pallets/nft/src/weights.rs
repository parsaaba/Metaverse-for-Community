// This file is part of Bit.Country.

// Copyright (C) 2020-2021 Bit.Country.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-10, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/metaverse-node
// benchmark
// --pallet=nft
// --extrinsic=*
// --steps=20
// --repeat=10
// --execution=wasm
// --wasm-execution=compiled
// --template=./template/weight-template.hbs
// --output
// ./pallets/nft/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for nft.
pub trait WeightInfo {
    fn create_group() -> Weight;
    fn create_class() -> Weight;
    fn mint(i: u32) -> Weight;
    fn transfer() -> Weight;
    fn transfer_batch(i: u32) -> Weight;
    fn sign_asset() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn create_group() -> Weight {
        (60_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn create_class() -> Weight {
        (225_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn mint(i: u32) -> Weight {
        (1_621_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((21_976_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
    }

    fn transfer() -> Weight {
        (176_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn transfer_batch(i: u32) -> Weight {
        (447_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(9 as Weight))
            .saturating_add(T::DbWeight::get().writes((9 as Weight).saturating_mul(i as Weight)))
    }
    fn sign_asset() -> Weight {
        (47_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn create_group() -> Weight {
        (60_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn create_class() -> Weight {
        (225_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn mint(i: u32) -> Weight {
        (1_621_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((21_976_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
    }
    fn transfer() -> Weight {
        (176_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn transfer_batch(i: u32) -> Weight {
        (447_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(8 as Weight))
            .saturating_add(RocksDbWeight::get().writes(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes((9 as Weight).saturating_mul(i as Weight)))
    }
    fn sign_asset() -> Weight {
        (47_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
