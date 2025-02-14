// Copyright 2024 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(allocator_api)]
#![feature(bound_as_ref)]
#![feature(bound_map)]
#![feature(custom_test_frameworks)]
#![feature(extract_if)]
#![feature(coroutines)]
#![feature(hash_extract_if)]
#![feature(lint_reasons)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(strict_provenance)]
#![feature(test)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(type_changing_struct_update)]
#![test_runner(risingwave_test_runner::test_runner::run_failpont_tests)]
#![feature(assert_matches)]
#![feature(is_sorted)]
#![feature(btree_extract_if)]
#![feature(exact_size_is_empty)]
#![feature(lazy_cell)]
#![cfg_attr(coverage, feature(coverage_attribute))]
#![recursion_limit = "256"]
#![feature(error_generic_member_access)]
#![feature(let_chains)]
#![feature(associated_type_bounds)]
#![feature(exclusive_range_pattern)]
#![feature(impl_trait_in_assoc_type)]

pub mod hummock;
pub mod memory;
pub mod monitor;
pub mod panic_store;
pub mod row_serde;
pub mod storage_value;
#[macro_use]
pub mod store;
pub mod error;
pub mod opts;
pub mod store_impl;
pub mod table;

pub mod filter_key_extractor;
pub mod mem_table;
#[cfg(test)]
#[cfg(feature = "failpoints")]
mod storage_failpoints;

pub use store::{StateStore, StateStoreIter, StateStoreReadIterStream};
pub use store_impl::StateStoreImpl;

pub enum TableScanOptions {
    SequentialScan,
    SparseIndexScan,
}
