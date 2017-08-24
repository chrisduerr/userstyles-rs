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
/// - Response is not `304`
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
    let url = [API_URL_BASE, &id.to_string()].concat();
    let mut resp = reqwest::get(&url)
        .map_err(|e| format!("Unable to make request to '{}': {}", url, e))?;

    let status = resp.status();
    if status != reqwest::StatusCode::Ok {
        Err(format!(
            "Error during API request. Expected status '304' but got '{}'",
            status.as_u16()
        ))
    } else {
        Ok(
            resp.json::<Style>()
                .map_err(|e| format!("Unable to parse json response: {}", e))?,
        )
    }
}

// pub fn get_css_with_settings(id: i32, settings: HashMap<String, String>) -> String {}
