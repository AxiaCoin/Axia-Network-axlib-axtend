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

//! Autogenerated weights for pallet_node_authorization

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_well_known_node() -> Weight;
	fn remove_well_known_node() -> Weight;
	fn swap_well_known_node() -> Weight;
	fn reset_well_known_nodes() -> Weight;
	fn claim_node() -> Weight;
	fn remove_claim() -> Weight;
	fn transfer_node() -> Weight;
	fn add_connections() -> Weight;
	fn remove_connections() -> Weight;
}

impl WeightInfo for () {
	fn add_well_known_node() -> Weight { 50_000_000 }
	fn remove_well_known_node() -> Weight { 50_000_000 }
	fn swap_well_known_node() -> Weight { 50_000_000 }
	fn reset_well_known_nodes() -> Weight { 50_000_000 }
	fn claim_node() -> Weight { 50_000_000 }
	fn remove_claim() -> Weight { 50_000_000 }
	fn transfer_node() -> Weight { 50_000_000 }
	fn add_connections() -> Weight { 50_000_000 }
	fn remove_connections() -> Weight { 50_000_000 }
}
