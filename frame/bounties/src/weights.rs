// This file is part of Axlib.

// Copyright (C) 2021 Axia Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_bounties
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/axlib
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/bounties/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bounties.
pub trait WeightInfo {
	fn propose_bounty(d: u32, ) -> Weight;
	fn approve_bounty() -> Weight;
	fn propose_curator() -> Weight;
	fn unassign_curator() -> Weight;
	fn accept_curator() -> Weight;
	fn award_bounty() -> Weight;
	fn claim_bounty() -> Weight;
	fn close_bounty_proposed() -> Weight;
	fn close_bounty_active() -> Weight;
	fn extend_bounty_expiry() -> Weight;
	fn spend_funds(b: u32, ) -> Weight;
}

/// Weights for pallet_bounties using the Axlib node and recommended hardware.
pub struct AxlibWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AxlibWeight<T> {
	// Storage: Treasury BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	// Storage: Treasury Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(44_482_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: Treasury BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(11_955_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(9_771_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(40_683_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(36_390_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn award_bounty() -> Weight {
		(25_187_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(124_785_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(39_483_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(83_453_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(24_151_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury BountyApprovals (r:1 w:1)
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((58_004_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Treasury BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	// Storage: Treasury Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(44_482_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: Treasury BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(11_955_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(9_771_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(40_683_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(36_390_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn award_bounty() -> Weight {
		(25_187_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(124_785_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(39_483_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(83_453_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(24_151_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury BountyApprovals (r:1 w:1)
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((58_004_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}
