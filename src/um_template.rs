/*
 * src/um_template.rs
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

// provides a template for the full username module
use askama::Template;

#[derive(Template)]
#[template(path = "../templates/username_module.j2", escape = "none")]
pub struct UserModuleTemplate<'a> {
    avatar_hover: &'a str,
    do_avatar_hover: bool,
    pfp_url: &'a str,
    userid: u64,
    username: &'a str,
}

impl<'a> UserModuleTemplate<'a> {
    pub fn new(userid: u64, username: &'a str, pfp_url: Option<&'a str>) -> UserModuleTemplate<'a> {
        let mut avatar_hover = "avatar_hover";
        let mut do_avatar_hover = true;
        let purl = match pfp_url {
            Some(u) => u,
            None => {
                avatar_hover = "";
                do_avatar_hover = false;
                ""
            }
        };

        // instantiate new object
        UserModuleTemplate {
            avatar_hover: avatar_hover,
            do_avatar_hover: do_avatar_hover,
            pfp_url: purl,
            userid: userid,
            username: username,
        }
    }
}

#[cfg(test)]
mod um_template_tests {
  use askama::Template;
  use super::UserModuleTemplate;

  #[test]
  fn no_pfp_test() {
    let um = UserModuleTemplate::new(399, "not_a_seagull", None);
    assert!(um.avatar_hover == "", "avatar_hover was not set to false");
    
    let text = um.render().unwrap();
    println!("{}", text);

    assert!(text.contains(r#"<a href="/sys/user-info/not_a_seagull" class="open-user-popup" id="399">not_a_seagull</a>"#));
  }

  #[test]
  fn pfp_test() {
    let um = UserModuleTemplate::new(399, "not_a_seagull", Some("/profile-pictures/399"));
    assert!(um.avatar_hover == "avatar_hover");

    let text = um.render().unwrap();
    println!("{}", text);
   
    assert!(text.contains(r#"<a href="/sys/user-info/not_a_seagull" class="open-user-popup" id="399">not_a_seagull</a>"#));
    assert!(text.contains(r#"<img class="small" src="/profile-pictures/399" alt="" />"#));
  }
}
