/*
 * src/lib.rs
 *
 * scp-username-module - Cross-language support for the username module
 * Copyright (C) 2019 not_a_seagull
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

extern crate askama;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// function to produce username module
#[wasm_bindgen(js_name = createUsernameModule)]
pub fn create_username_module(
  username: &str,
  profile_picture_url: &str,
  is_pro: bool
) -> Option<String> {
  None
}
