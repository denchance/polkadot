// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::paras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_paras.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras::WeightInfo for WeightInfo<T> {
	// Storage: Paras CurrentCodeHash (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras PastCodeMeta (r:1 w:1)
	// Storage: Paras PastCodePruning (r:1 w:1)
	// Storage: Paras PastCodeHash (r:0 w:1)
	// Storage: Paras CodeByHash (r:0 w:1)
	fn force_set_current_code(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Paras Heads (r:0 w:1)
	fn force_set_current_head(s: u32, ) -> Weight {
		(9_108_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:1)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras UpgradeCooldowns (r:1 w:1)
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	// Storage: Paras UpcomingUpgrades (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	// Storage: Paras UpgradeRestrictionSignal (r:0 w:1)
	fn force_schedule_code_upgrade(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn force_note_new_head(s: u32, ) -> Weight {
		(12_655_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	fn force_queue_action() -> Weight {
		(18_727_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	fn add_trusted_validation_code(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Paras CodeByHashRefs (r:1 w:0)
	// Storage: Paras CodeByHash (r:0 w:1)
	fn poke_unused_validation_code() -> Weight {
		(2_668_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	fn include_pvf_check_statement() -> Weight {
		(101_638_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras UpcomingUpgrades (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:0 w:100)
	fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
		(685_517_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(104 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras CodeByHash (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:100)
	// Storage: Paras FutureCodeHash (r:0 w:100)
	fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
		(588_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(204 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras ActionsQueue (r:1 w:1)
	fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
		(545_362_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:0 w:100)
	// Storage: Paras CodeByHash (r:0 w:1)
	// Storage: Paras CurrentCodeHash (r:0 w:100)
	// Storage: Paras UpcomingParasGenesis (r:0 w:100)
	fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
		(645_837_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(304 as Weight))
	}
}
