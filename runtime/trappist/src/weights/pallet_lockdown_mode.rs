// This file is part of Trappist.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_lockdown_mode`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-aahe6cbd-project-647-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("trappist-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/trappist-node
// benchmark
// pallet
// --chain=trappist-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_lockdown_mode
// --extrinsic=*
// --wasm-execution=compiled
// --header=./templates/file_header.txt
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_lockdown_mode`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_lockdown_mode::WeightInfo for WeightInfo<T> {
	/// Storage: `LockdownMode::LockdownModeStatus` (r:1 w:1)
	/// Proof: `LockdownMode::LockdownModeStatus` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::QueueSuspended` (r:0 w:1)
	/// Proof: `XcmpQueue::QueueSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn activate_lockdown_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `1486`
		// Minimum execution time: 14_323_000 picoseconds.
		Weight::from_parts(14_782_000, 0)
			.saturating_add(Weight::from_parts(0, 1486))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LockdownMode::LockdownModeStatus` (r:1 w:1)
	/// Proof: `LockdownMode::LockdownModeStatus` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::QueueSuspended` (r:0 w:1)
	/// Proof: `XcmpQueue::QueueSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn deactivate_lockdown_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `1486`
		// Minimum execution time: 14_139_000 picoseconds.
		Weight::from_parts(14_669_000, 0)
			.saturating_add(Weight::from_parts(0, 1486))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
