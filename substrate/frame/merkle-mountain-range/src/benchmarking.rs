// This file is part of Substrate.

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

//! Benchmarks for the MMR pallet.

#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame::{
	benchmarking::prelude::v1::benchmarks_instance_pallet,
	deps::frame_support::traits::OnInitialize,
};

benchmarks_instance_pallet! {
	on_initialize {
		let x in 1 .. 1_000;

		let leaves = x as NodeIndex;

		<<T as pallet::Config::<I>>::BenchmarkHelper as BenchmarkHelper>::setup();
		for leaf in 0..(leaves - 1) {
			<Pallet::<T, I> as OnInitialize<BlockNumberFor<T>>>::on_initialize((leaf as u32).into());
		}
	}: {
		<Pallet::<T, I> as OnInitialize<BlockNumberFor<T>>>::on_initialize((leaves as u32 - 1).into());
	} verify {
		assert_eq!(crate::NumberOfLeaves::<T, I>::get(), leaves);
	}

	impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::mock::Test);
}
