# unic-locale [![Build Status](https://travis-ci.org/zbraniecki/unic-locale.svg?branch=master)](https://travis-ci.org/zbraniecki/unic-locale) [![Coverage Status](https://coveralls.io/repos/github/zbraniecki/unic-locale/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/unic-locale?branch=master)

`unic-locale` is an API for managing [Unicode Locale Identifiers](http://unicode.org/reports/tr35/#Unicode_locale_identifier).

The crate provides a way to create a struct from a string, manipulate its fields, canonicalize it, and serialize into a string.

Usage
-----

```rust
use unic_locale::{Locale, ExtensionType};

let loc = Locale::from_str("en-US-u-hc-h12");

assert_eq!(loc.get_language(), "en");
assert_eq!(loc.get_script(), None);
assert_eq!(loc.get_region(), Some("US"));

loc.set_extension(ExtensionType::Unicode, "calendar", "buddhist");

assert_eq!(&loc.to_string(), "en-US-u-ca-buddhist-hc-h12");
```

Status
------

The crate is providing fundamental blocks, but is very basic.

Get Involved
------------

`unic-locale` is open-source, licensed under the Apache License, Version 2.0.  We
encourage everyone to take a look at our code and we'll listen to your
feedback.
