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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/axlib
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/utility/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
}

/// Weights for pallet_utility using the Axlib node and recommended hardware.
pub struct AxlibWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AxlibWeight<T> {
	fn batch(c: u32, ) -> Weight {
		(18_293_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_530_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(3_387_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(19_223_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((5_998_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(14_340_000 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn batch(c: u32, ) -> Weight {
		(18_293_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_530_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(3_387_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(19_223_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((5_998_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(14_340_000 as Weight)
	}
}
