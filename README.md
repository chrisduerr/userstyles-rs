## Userstyles

This are API bindings for the simple `userstyles.org` API. It makes it possible to download styles with or without parameters.

Here's how you can get an API response with all metadata about a style:

```
use userstyles::get_style;

// Style URL: "https://userstyles.org/styles/37035/github-dark"
let response = get_style(37035);
```

This will allow you to access the complete API response. If you want to just get the plain default css, all it takes is this:

```
...
response.unwrap().css;
```

In case you want the css with different settings you will have to send another request. First take the settings you want to modify out of `Style.settings`. Then create a hashmap with all the settings you want to change.
After that you're only one request away from the css.

```
...
let css = get_css(37035, settings_map);
```
