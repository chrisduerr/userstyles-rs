# Userstyles [![Build Status](https://img.shields.io/crates/v/rustc-serialize.svg?style=flat-square)](https://crates.io/crates/userstyles)

#### [Documentation](https://docs.rs/userstyles)

`Userstyles` provides API bindings for `userstyles.org`.
This makes it possible to get styles, their settings and other metadata.

For getting all information about a style you can use `get_style`.
This provides a struct with most informations about a style,
however it does not allow you to get a style's css with custom settings.

```rust
use userstyles::get_style;

// Style URL: "https://userstyles.org/styles/37035/github-dark"
let style = get_style(37035);
```

If you want to just access the css with the default settings you can
use the `css` property of the response.

```rust
use userstyles::get_style;

// Style URL: "https://userstyles.org/styles/37035/github-dark"
let style = get_style(37035).unwrap();

let css = style.css;
```

If you are only interested in the css, but want to change the settings, you can use `get_css`.
This takes a `HashMap` with all keys and values you want to set.
You can get the available settings from `Style.style_settings` after using `get_style`.

The API requires all keys and values to start with `ik-`, so this is added automatically by `get_css`.
The `install_key` of `StyleSetting` and `StyleSettingOption` does not start with
`ik-` and work without modification. But when getting keys and values from differen sources,
please make sure they do not start with `ik-`.

```rust
use std::collections::HashMap;
use userstyles::get_css;

let mut settings = HashMap::new();
settings.insert(String::from("ACCENTCOLOR"), String::from("#f006a2"));
let css = get_css(37035, &settings).unwrap();
```
