# Userstyles [![Build Status](https://img.shields.io/crates/v/userstyles.svg?style=flat-square)](https://crates.io/crates/userstyles)

#### [Documentation](https://docs.rs/userstyles)

`Userstyles` provides API bindings for `userstyles.org`.
This makes it possible to get styles, their settings and other metadata.

For getting all information about a style you can use `get_style`.

```rust
use userstyles::get_style;

// Style URL: "https://userstyles.org/styles/37035/github-dark"
let style = get_style(37035);
```

If you just want to access the css with the default settings you can
use the `get_css` method with `None` as parameter.

```rust
use userstyles::response::Style;

let style = Style::default();

let css = style.get_css(None);
```

If you are interested in the css, but want to change the settings,
you can also use `get_css`. This takes a `HashMap` with all keys and values you want to set.
You can get all the available settings from `Style.style_settings`.

```rust
use userstyles::response::Style;
use std::collections::HashMap;

let style = Style::default();
let mut map = HashMap::new();
map.insert(String::from("ACCENTCOLOR"), String::from("#f00ba2"));

let css = style.get_css(Some(map));
```
