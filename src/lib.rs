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
/// let response = get_css(37035, HashMap::new());
/// assert!(response.is_ok());
/// ```
pub fn get_css(id: i32, settings: HashMap<String, String>) -> Result<String, String> {
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
