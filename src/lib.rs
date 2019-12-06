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
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

mod um_template;

use self::um_template::UserModuleTemplate;
use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::prelude::*;

/// Generates a username module from the user's id, name, and optionally their profile picture URL
#[wasm_bindgen(js_name = createUsernameModule)]
pub fn create_username_module(
    userid: u64,
    username: String,
    profile_picture_url: Option<String>,
) -> Option<String> {
    let purl: Option<&str> = match profile_picture_url {
        Some(ref p) => Some(&p),
        None => None,
    };

    match UserModuleTemplate::new(userid, &username, purl).render() {
        Ok(x) => Some(x),
        Err(e) => {
            eprintln!("An error occurred while rendering the user module: {}", e);
            None
        }
    }
}

/// A username module to deserialize from or to serialize to
#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Serialize)]
pub struct UsernameModule {
    pub userid: u64,
    pub username: String,
    pub profile_picture_url: Option<String>,
}

/// Deserialize a username module into a string from JSON
#[wasm_bindgen(js_name = deserializeUsernameModule)]
pub fn deserialize_username_module(json: &str) -> Option<String> {
    match from_str::<UsernameModule>(json) {
        Ok(module) => {
            create_username_module(module.userid, module.username, module.profile_picture_url)
        }
        Err(e) => {
            eprintln!("An error occurred while deserializing from JSON: {}", e);
            None
        }
    }
}
