// Copyright 2019-2020
//     by  Centrality Investments Ltd.
//     and Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

//! Efficient and compact serialization of Rust types.
//!
//! This library provides structures to easily retrieve compile-time type
//! information at runtime and also to serialize this information in a compact
//! form.
//!
//! # Registry
//!
//! At the heart of its functionality is the `Registry` that acts as cache for
//! known strings and types in order to efficiently deduplicate them and thus
//! compactify the overall serialization.
//!
//! # Type Information
//!
//! Information about types is provided via the `TypeInfo` trait.
//!
//! This trait should be implemented for all types that are serializable.
//! For this the library provides implementations for all commonly used Rust
//! standard types and provides derive macros for simpler implementation of user
//! provided custom types.
//!
//! # Compaction Forms
//!
//! There is an uncompact form, called `MetaForm` that acts as a bridge from
//! compile-time type information at runtime in order to easily retrieve all
//! information needed to uniquely identify types.
//! The compact form is retrieved by the `IntoCompact` trait and internally used
//! by the `Registry` in order to convert the uncompact strings and types into
//! their compact form.
//!
//! # Symbols and Namespaces
//!
//! Since symbol names are often shared across type boundaries the `Registry`
//! also deduplicates them. To differentiate two types sharing the same name
//! namespaces are used. Commonly the namespace is equal to the one where the
//! type has been defined in. For Rust prelude types such as `Option` and
//! `Result` the root namespace (empty namespace) is used.
//!
//! To use this library simply use the `MetaForm` initially with your own data
//! structures and at best make them generic over the `Form` trait just as has
//! been done in this crate with `TypeInfo` in order to go for a simple
//! implementation of `IntoCompact`. Use a single instance of the `Registry` for
//! compaction and provide this registry instance upon serialization. Done.
//!
//! A usage example can be found in ink! here:
//! https://github.com/paritytech/ink/blob/master/abi/src/specs.rs

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_export]
macro_rules! type_param {
	( $ty:ty ) => {
		$crate::MetaTypeParameter::new::<Self, $ty>(stringify!($ty)).into()
	};
}

#[macro_export]
macro_rules! type_params {
	( $($ty:ty),* ) => {
		{
			#[cfg(not(feature = "std"))]
			extern crate alloc as _alloc;
			#[cfg(not(feature = "std"))]
			#[allow(unused_mut)]
			let mut v = _alloc::vec![];

			#[cfg(feature = "std")]
			#[allow(unused_mut)]
			let mut v = std::vec![];

			$(
				v.push($crate::type_param!($ty));
			)*
			v
		}
	}
}

mod tm_std;

pub mod form;
mod impls;
mod meta_type;
mod registry;
mod ty;
mod type_id;
mod utils;

#[cfg(test)]
mod tests;

pub use self::{
	meta_type::{MetaType, MetaTypeParameter, MetaTypeParameterValue},
	registry::{IntoCompact, Registry, InternedTypeParameter, interner::UntrackedSymbol},
	ty::*,
	type_id::*,
};

#[cfg(feature = "derive")]
pub use scale_info_derive::Metadata;

/// A super trait that shall be implemented by all types implementing `TypeInfo`
pub trait Metadata: TypeInfo {
	/// Returns the runtime bridge to the types compile-time type information.
	fn meta_type() -> MetaType;
}

/// Implementors return their meta type information.
/// todo: [AJ] consider splitting to separate traits?
pub trait TypeInfo {
	fn path() -> Path;

	fn params() -> Vec<MetaTypeParameter> {
		Vec::new()
	}

	fn type_info() -> Type;
}

impl<T> Metadata for T
where
	T: ?Sized + TypeInfo + 'static,
{
	fn meta_type() -> MetaType {
		MetaType::concrete::<T>()
	}
}
