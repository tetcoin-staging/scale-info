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

use crate::tm_std::*;

use crate::{
	form::{CompactForm, Form, MetaForm},
	state, Field, FieldsBuilder, IntoCompact, MetaType, Path, PathError, Registry,
};
use derive_more::From;
use serde::Serialize;

/// A composite type, consisting of either named (struct) or unnamed (tuple
/// struct) fields
///
/// # Examples
///
/// ## A Rust struct with named fields.
///
/// ```
/// struct Person {
///     name: String,
///     age_in_years: u8,
///     friends: Vec<Person>,
/// }
/// ```
///
/// ## A tuple struct with unnamed fields.
///
/// ```
/// struct Color(u8, u8, u8);
/// ```
///
/// ## A so-called unit struct
///
/// ```
/// struct JustAMarker;
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, From)]
#[serde(bound = "F::TypeId: Serialize")]
#[serde(rename_all = "lowercase")]
pub struct TypeComposite<F: Form = MetaForm> {
	#[serde(skip_serializing_if = "Vec::is_empty")]
	fields: Vec<Field<F>>,
}

impl IntoCompact for TypeComposite {
	type Output = TypeComposite<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		TypeComposite {
			fields: registry.map_into_compact(self.fields),
		}
	}
}

impl TypeComposite {
	/// Creates a new struct definition with named fields.
	pub fn new(fields: FieldsBuilder<F>) -> Self {
		Self {
			fields: fields.done()
		}
	}

	pub fn unit() -> Self {
		Self {
			fields: Vec::new()
		}
	}
}