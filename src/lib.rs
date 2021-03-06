// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
#![deny(missing_docs)]

//! Efficient and space-efficient serialization of Rust types.
//!
//! This library provides structures to easily retrieve compile-time type
//! information at runtime and also to serialize this information in a
//! space-efficient form, aka `PortableForm`.
//!
//! # Registry
//!
//! At the heart of its functionality is the [`Registry`](`crate::Registry`)
//! that acts as a cache for known types in order to efficiently deduplicate
//! types and ensure a space-efficient serialization.
//!
//! # Type Information
//!
//! Information about types is provided via the [`TypeInfo`](`crate::TypeInfo`)
//! trait.
//!
//! This trait should be implemented for all types that are serializable.
//! `scale-info` provides implementations for all commonly used Rust standard
//! types and a derive macro for implementing of custom types.
//!
//! # Forms
//!
//! To bridge between compile-time type information and runtime the
//! [`MetaForm`](`crate::form::MetaForm`) is used to easily retrieve all
//! information needed to uniquely identify types.
//!
//! The `MetaForm` and its associated `Registry` can be transformed into the
//! space-efficient form by the [`IntoPortable`](`crate::IntoPortable`) trait; it is
//! used internally by the [`Registry`](`crate::Registry`) in order to convert
//! the expanded types into their space-efficient form.
//!
//! # Symbols and Namespaces
//!
//! To differentiate two types sharing the same name, namespaces are used.
//! Commonly the namespace is equal to the one where the type has been defined
//! in. For Rust prelude types such as [`Option`](`std::option::Option`) and
//! [`Result`](`std::result::Result`) the root namespace (empty namespace) is
//! used.
//!
//! To use this library simply use the [`MetaForm`](`crate::form::MetaForm`)
//! initially with your own data structures; make them generic over the
//! [`Form`](`crate::form::Form`) trait just as has been done in this crate with
//! [`TypeInfo`](`crate::TypeInfo`) in order to get a simple implementation of
//! [`IntoPortable`](`crate::IntoPortable`). Use a single instance of the
//! [`Registry`](`crate::Registry`) for compaction and provide this registry
//! instance upon serialization.
//!
//! A usage example can be found in ink! here:
//! https://github.com/paritytech/ink/blob/master/abi/src/specs.rs

/// Takes a number of types and returns a vector that contains their respective
/// [`MetaType`](`crate::MetaType`) instances.
///
/// This is useful for places that require inputs of iterators over [`MetaType`](`crate::MetaType`)
/// instances and provide a way out of code bloat in these scenarious.
///
/// # Example
///
/// ```
/// # use scale_info::tuple_meta_type;
/// assert_eq!(
///     tuple_meta_type!(i32, [u8; 32], String),
///     {
///         use scale_info::MetaType;
///         let mut vec = Vec::new();
///         vec.push(MetaType::new::<i32>());
///         vec.push(MetaType::new::<[u8; 32]>());
///         vec.push(MetaType::new::<String>());
///         vec
///     }
/// );
/// ```
#[macro_export]
macro_rules! tuple_meta_type {
    ( $($ty:ty),* ) => {
        {
            $crate::prelude::vec![
                $(
                    $crate::MetaType::new::<$ty>(),
                )*
            ]
        }
    }
}

pub mod prelude;

pub mod build;
pub mod form;
mod impls;
pub mod interner;
mod meta_type;
mod registry;
mod ty;
mod utils;

#[cfg(test)]
mod tests;

pub use self::{
    meta_type::MetaType,
    registry::{
        IntoPortable,
        PortableRegistry,
        Registry,
    },
    ty::*,
};

#[cfg(feature = "derive")]
pub use scale_info_derive::TypeInfo;

/// Implementors return their meta type information.
pub trait TypeInfo {
    /// The type identifying for which type info is provided.
    ///
    /// # Note
    ///
    /// This is used to uniquely identify a type via [`core::any::TypeId::of`]. In most cases it
    /// will just be `Self`, but can be used to unify different types which have the same encoded
    /// representation e.g. reference types `&T` and `&mut T`.
    type Identity: ?Sized + 'static;

    /// Returns the static type identifier for `Self`.
    fn type_info() -> Type;
}

/// Returns the runtime bridge to the types compile-time type information.
pub fn meta_type<T>() -> MetaType
where
    T: ?Sized + TypeInfo + 'static,
{
    MetaType::new::<T>()
}
