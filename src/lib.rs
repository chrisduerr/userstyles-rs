//! # Userstyles
//!
//! `Userstyles` provides API bindings for `userstyles.org`.
//! This makes it possible to get styles, their settings and other metadata.
//!
//! For getting all information about a style you can use [`get_style`](fn.get_style.html).
//!
//! ```rust
//! use userstyles::get_style;
//!
//! // Style URL: "https://userstyles.org/styles/37035/github-dark"
//! let style = get_style(37035);
//! ```
//!
//! If you just want to access the css with the default settings you can
//! use the [`get_css`](response/struct.Style.html#method.get_css) method with `None` as parameter.
//!
//! ```rust
//! use userstyles::response::Style;
//!
//! let style = Style::default();
//!
//! let css = style.get_css(None);
//! ```
//!
//! If you are interested in the css, but want to change the settings,
//! you can also use [`get_css`](response/struct.Style.html#method.get_css).
//! This takes a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
//! with all keys and values you want to set. You can get all the available settings from
//! [`Style.style_settings`](response/struct.Style.html#fields).
//!
//! The API requires all keys to start with `ik-`, so this is added automatically by
//! [`get_css`](response/struct.Style.html#method.get_css). The `install_key` of
//! [`StyleSetting`](response/struct.StyleSetting.html#fields) does not start with
//! `ik-` and works without modification. But when getting keys from differen sources,
//! please make sure they do not start with `ik-`.
//!
//! ```rust
//! use userstyles::response::Style;
//! use std::collections::HashMap;
//!
//! let style = Style::default();
//! let mut map = HashMap::new();
//! map.insert(String::from("ACCENTCOLOR"), String::from("#f00ba2"));
//!
//! let css = style.get_css(Some(map));
//! ```

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate serde;

use response::Style;

pub mod response;

const API_URL_BASE: &str = "https://userstyles.org/api/v1/styles/";

/// Make a request to the API for a style.
/// `id` is the identifier of the style you want to request.
///
/// # Panics
///
/// Panics under the following conditions:
///
/// - Unable to create a request
/// - Response is not `200`
/// - The response could not be parsed
///
/// # Examples
///
/// ```
/// use userstyles::get_style;
///
/// // Style URL: "https://userstyles.org/styles/37035/github-dark"
/// let response = get_style(37035);
/// assert!(response.is_ok());
/// ```
pub fn get_style(id: u32) -> Result<Style, String> {
    // Construct request
    let url = [API_URL_BASE, &id.to_string()].concat();
    let mut resp = reqwest::get(&url)
        .map_err(|e| format!("Unable to make request to '{}': {}", url, e))?;

    // Check if status is ok
    let status = resp.status();
    if status != reqwest::StatusCode::Ok {
        Err(format!(
            "Error during API request. Expected status '200' but got '{}'",
            status.as_u16()
        ))
    } else {
        // Parse response json
        Ok(
            resp.json::<Style>()
                .map_err(|e| format!("Unable to parse json response: {}", e))?,
        )
    }
}
