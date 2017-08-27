use std::collections::HashMap;

/// This struct is the root of the standard userstyles API response.
///
/// The request url is `https://userstyles.org/api/v1/styles/{id}`.
#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct Style {
    /// id of the style, this is part of the `userstyles.org` url
    pub id: i32,
    /// Name of the style
    pub name: String,
    /// Summary of what the style does
    pub description: String,
    /// User that created the style
    pub user: User,
    /// Last update time
    pub updated: String,
    /// Installs per this week
    pub weekly_install_count: i32,
    /// Total install count
    pub total_install_count: i32,
    /// Rating of this style from 1 to 3
    pub rating: Option<f32>,
    /// File name of the thumbnail
    pub after_screenshot_name: Option<String>,
    /// id for newer version of this style
    pub obsoleting_style_id: Option<i32>,
    /// Name of the replacing style
    pub obsoleting_style_name: Option<String>,
    /// Indicate that style has been discontinued
    pub obsolete: u8,
    /// Reason why style has been removed by an admin
    pub admin_delete_reason: Option<String>,
    /// Reason why style has been obsoleted
    pub obsoletion_message: Option<String>,
    /// Screenshot fil names for this style
    pub screenshots: Option<Vec<String>>,
    /// License the style is published under
    pub license: Option<String>,
    /// Creation time
    pub created: String,
    /// Category this style falls in
    pub category: String,
    /// Subcategory or domain name
    pub subcategory: Option<String>,
    /// Pleadgie id, mostly unused
    pub pledgie_id: Option<i32>,
    /// Additional informations about this style
    pub additional_info: Option<String>,
    /// The style's css with template placeholders
    pub css: String,
    /// Comments on this style
    pub discussions: Vec<Discussion>,
    /// JavaScipt file name for this style
    pub userjs_url: Option<String>,
    /// Available settings
    pub style_settings: Vec<StyleSetting>,
}

/// `userstyles.org` user.
#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct User {
    /// id of the user
    pub id: i32,
    /// Username
    pub name: String,
    /// Email address
    pub email: Option<String>,
    /// Paypal email
    pub paypal_email: Option<String>,
    /// Homepage
    pub homepage: Option<String>,
    /// Bio about the user
    pub about: Option<String>,
    /// Default license
    pub license: Option<String>,
}

/// Single comment about a userstyle
#[derive(Deserialize, PartialEq, Debug)]
pub struct Discussion {
    /// Comment id
    pub id: i32,
    /// Comment text
    pub name: String,
    /// Rating either 0, 1, 2 or 3.
    /// 0 means no rating was given.
    pub rating: i32,
    /// Creation date of this comment
    pub created: String,
    /// Username of the comment author
    pub author_name: String,
    /// User id of the comment author
    pub author_id: i32,
}

/// Available option for a userstyle
#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct StyleSetting {
    /// id of this setting
    pub id: i32,
    /// id of style this setting belongs to
    pub style_id: i32,
    /// key for request body
    pub install_key: String,
    /// Human-readable name of this setting
    pub label: String,
    /// The type of this setting.
    /// This is eiter `color`, `image`, `text` or `dropdown`.
    pub setting_type: String,
    /// The available options and default
    pub style_setting_options: Vec<StyleSettingOption>,
}

/// Available options and default for a setting
#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct StyleSettingOption {
    /// id of this option
    pub id: i32,
    /// id of the setting this option belongs to
    pub style_setting_id: i32,
    /// Human-readable name of this option
    pub label: String,
    /// Text that will be replace the template
    pub value: String,
    /// Indicate that this is the default option
    pub default: bool,
    /// Order id for arranging options
    pub ordinal: i32,
    /// value for request body
    pub install_key: String,
}

impl Style {
    // Get a HashMap with all the default setting keys and vals
    fn get_default_settings(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for setting in &self.style_settings {
            for option in &setting.style_setting_options {
                if option.default {
                    map.insert(setting.install_key.clone(), option.value.clone());
                    break;
                }
            }
        }

        map
    }

    /// Get the CSS of a style. This uses the style's default settings unless
    /// `settings` exists and provides an override. The elements of `settings`
    /// are always prioritized over the defaults.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use userstyles::response::Style;
    ///
    /// let style = Style::default();
    /// let mut map = HashMap::new();
    /// map.insert(String::from("ACCENTCOLOR"), String::from("#f00ba2"));
    ///
    /// let css = style.get_css(Some(map));
    /// ```
    pub fn get_css(&self, settings: Option<HashMap<String, String>>) -> String {
        // If `settings` is none, create an empty map
        let settings = if let Some(map) = settings {
            map
        } else {
            HashMap::new()
        };

        // Create return value
        let mut css = self.css.clone();

        // Get default settings
        let defaults = self.get_default_settings();

        // Iterate over all settings
        for (key, mut val) in &defaults {
            // Replace defaults with user overrides
            if let Some(value) = settings.get(key) {
                val = value;
            }

            // Make key into template
            let key = ["/*[[", key, "]]*/"].concat();

            // Replace all instances of `key` with `val`
            css = css.replace(&key, val);
        }

        // Return the rendered CSS
        css
    }
}
