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

//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_identity
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 12_498_000 picoseconds.
		Weight::from_parts(13_282_682, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_684
			.saturating_add(Weight::from_parts(87_391, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442 + r * (5 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 32_268_000 picoseconds.
		Weight::from_parts(30_589_814, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 3_776
			.saturating_add(Weight::from_parts(125_849, 0).saturating_mul(r.into()))
			// Standard Error: 736
			.saturating_add(Weight::from_parts(463_056, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `11003 + s * (2589 ±0)`
		// Minimum execution time: 9_105_000 picoseconds.
		Weight::from_parts(22_301_022, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 4_388
			.saturating_add(Weight::from_parts(3_119_894, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194 + p * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 9_015_000 picoseconds.
		Weight::from_parts(22_372_621, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 3_467
			.saturating_add(Weight::from_parts(1_349_643, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 55_487_000 picoseconds.
		Weight::from_parts(32_144_815, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 11_235
			.saturating_add(Weight::from_parts(142_586, 0).saturating_mul(r.into()))
			// Standard Error: 2_194
			.saturating_add(Weight::from_parts(1_358_471, 0).saturating_mul(s.into()))
			// Standard Error: 2_194
			.saturating_add(Weight::from_parts(247_188, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `367 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 32_557_000 picoseconds.
		Weight::from_parts(30_635_170, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 4_595
			.saturating_add(Weight::from_parts(138_114, 0).saturating_mul(r.into()))
			// Standard Error: 896
			.saturating_add(Weight::from_parts(479_756, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398 + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 29_094_000 picoseconds.
		Weight::from_parts(28_341_732, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 3_290
			.saturating_add(Weight::from_parts(95_817, 0).saturating_mul(r.into()))
			// Standard Error: 642
			.saturating_add(Weight::from_parts(474_214, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_634_000 picoseconds.
		Weight::from_parts(8_077_478, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_633
			.saturating_add(Weight::from_parts(79_445, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 6_836_000 picoseconds.
		Weight::from_parts(7_257_400, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_034
			.saturating_add(Weight::from_parts(68_549, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 6_751_000 picoseconds.
		Weight::from_parts(7_303_990, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_047
			.saturating_add(Weight::from_parts(69_351, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 22_444_000 picoseconds.
		Weight::from_parts(21_599_992, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 4_345
			.saturating_add(Weight::from_parts(87_256, 0).saturating_mul(r.into()))
			// Standard Error: 803
			.saturating_add(Weight::from_parts(774_101, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `643 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 61_956_000 picoseconds.
		Weight::from_parts(38_393_402, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 10_135
			.saturating_add(Weight::from_parts(138_471, 0).saturating_mul(r.into()))
			// Standard Error: 1_979
			.saturating_add(Weight::from_parts(1_332_992, 0).saturating_mul(s.into()))
			// Standard Error: 1_979
			.saturating_add(Weight::from_parts(255_737, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `475 + s * (36 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 29_061_000 picoseconds.
		Weight::from_parts(33_042_875, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 975
			.saturating_add(Weight::from_parts(88_595, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591 + s * (3 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 12_260_000 picoseconds.
		Weight::from_parts(13_724_359, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 447
			.saturating_add(Weight::from_parts(38_060, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638 + s * (35 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 31_985_000 picoseconds.
		Weight::from_parts(34_715_046, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 971
			.saturating_add(Weight::from_parts(83_313, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `667 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 24_154_000 picoseconds.
		Weight::from_parts(26_248_523, 0)
			.saturating_add(Weight::from_parts(0, 6723))
			// Standard Error: 696
			.saturating_add(Weight::from_parts(74_385, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
