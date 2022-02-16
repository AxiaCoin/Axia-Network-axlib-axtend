// This file is part of Axlib.

// Copyright (C) 2017-2021 Axia Technologies (UK) Ltd.
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

//! Declaration of store trait and implementation on module structure.

use super::DeclStorageDefExt;
use proc_macro2::TokenStream;
use quote::quote;

pub fn decl_and_impl(def: &DeclStorageDefExt) -> TokenStream {
	let decl_store_items = def.storage_lines.iter().map(|sline| &sline.name).fold(
		TokenStream::new(),
		|mut items, name| {
			items.extend(quote!(type #name;));
			items
		},
	);

	let impl_store_items = def.storage_lines.iter().fold(TokenStream::new(), |mut items, line| {
		let name = &line.name;
		let storage_struct = &line.storage_struct;

		items.extend(quote!(type #name = #storage_struct;));
		items
	});

	let visibility = &def.visibility;
	let store_trait = &def.store_trait;
	let module_struct = &def.module_struct;
	let module_impl = &def.module_impl;
	let where_clause = &def.where_clause;

	quote!(
		#visibility trait #store_trait {
			#decl_store_items
		}
		impl #module_impl #store_trait for #module_struct #where_clause {
			#impl_store_items
		}
	)
}
