//! # Userstyles
//!
//! `Userstyles` provides API bindings for `userstyles.org`.
//! This makes it possible to get styles, their settings and other metadata.
//!
//! For getting all information about a style you can use [`get_style`](fn.get_style.html).
//! This provides a struct with most informations about a style,
//! however it does not allow you to get a style's css with custom settings.
//!
//! ```rust
//! use userstyles::get_style;
//!
//! // Style URL: "https://userstyles.org/styles/37035/github-dark"
//! let style = get_style(37035);
//! ```
//!
//! If you want to just access the css with the default settings you can
//! use the `css` property of the response.
//!
//! ```rust
//! use userstyles::get_style;
//!
//! // Style URL: "https://userstyles.org/styles/37035/github-dark"
//! let style = get_style(37035).unwrap();
//!
//! let css = style.css;
//! ```
//!
//! If you are only interested in the css, but want to change the settings,
//! you can use [`get_css`](fn.get_css.html).
//! This takes a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
//! with all keys and values you want to set. You can get the available settings from
//! [`Style.style_settings`](response/struct.Style.html#fields)
//! after using [`get_style`](fn.get_style.html).
//!
//! The API requires all keys and values to start with `ik-`, so this is added automatically by
//! [`get_css`](fn.get_css.html). The `install_key` of
//! [`StyleSetting`](response/struct.StyleSetting.html) and
//! [`StyleSettingOption`](response/struct.StyleSettingOption.html) does not start with
//! `ik-` and work without modification. But when getting keys and values from differen sources,
//! please make sure they do not start with `ik-`.
//!
//! ```rust
//! use std::collections::HashMap;
//! use userstyles::get_css;
//!
//! let mut settings = HashMap::new();
//! settings.insert(String::from("ACCENTCOLOR"), String::from("#f006a2"));
//! let css = get_css(37035, &settings).unwrap();
//! ```

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use response::Style;
use std::io::Read;

pub mod response;

const API_URL_BASE: &str = "https://userstyles.org/api/v1/styles/";
const CSS_URL_BASE: &str = "https://userstyles.org/styles/";

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

/// Make a request to the API for a specific stylesheet.
/// `id` is the identifier of the style you want the stylesheet for.
/// `settings` is the request body.
///
/// This automatically prepends `ik-` to every key and value.
/// So `ACCENTCOLOR` will be `ik-ACCENTCOLOR`. This is required by the API.
///
/// # Panics
///
/// - Unable to create a request
/// - Response is not `200`
/// - The response could not be parsed
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use userstyles::get_css;
///
/// // Style URL: "https://userstyles.org/styles/37035/github-dark"
/// let response = get_css(37035, &HashMap::new());
/// assert!(response.is_ok());
/// ```
pub fn get_css(id: i32, settings: &HashMap<String, String>) -> Result<String, String> {
    // Create body from map
    let mut request_body = String::new();
    for (key, val) in settings {
        request_body.push_str(&["ik-", &key, "=ik-", &val, "&"].concat());
    }
    let _ = request_body.pop();

    // Construct the query
    let client = reqwest::Client::new()
        .map_err(|e| format!("Unable to create request client: {}", e))?;
    let url = [CSS_URL_BASE, &id.to_string(), ".css"].concat();
    let mut resp = client
        .post(&url)
        .map_err(|e| format!("Unable to create post request: {}", e))?
        .body(request_body)
        .send()
        .map_err(|e| format!("Unable to sen post request: {}", e))?;

    // Check if response is ok
    let status = resp.status();
    if status != reqwest::StatusCode::Ok {
        Err(format!(
            "Error during API request. Expected status '200' but got '{}'",
            status.as_u16()
        ))
    } else {
        // Parse response as String and return it
        let mut buf = String::new();
        resp.read_to_string(&mut buf)
            .map_err(|e| format!("Unable to parse response: {}", e))?;
        Ok(buf)
    }
}
