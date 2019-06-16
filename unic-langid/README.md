# unic-langid [![Build Status](https://travis-ci.org/zbraniecki/unic-locale.svg?branch=master)](https://travis-ci.org/zbraniecki/unic-locale) [![Coverage Status](https://coveralls.io/repos/github/zbraniecki/unic-locale/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/unic-locale?branch=master)

`unic-langid` is an API for managing [Unicode Language Identifiers](http://unicode.org/reports/tr35/#Unicode_language_identifier).

The crate provides a way to create a struct from a string, manipulate its fields, canonicalize it, and serialize into a string.

Usage
-----

```rust
use unic_langid::LanguageIdentifier;

let loc = LanguageIdentifier::from_str("en-US");
assert_eq!(loc.get_language(), "en");
assert_eq!(loc.get_script(), None);
assert_eq!(loc.get_region(), Some("US"));

loc.set_script(Some("Latn"));

assert_eq!(&loc.to_string(), "en-Latn-US");
```

Status
------

The crate is providing fundamental blocks, but is very basic.

Get Involved
------------

`unic-langid` is open-source, licensed under the Apache License, Version 2.0.  We
encourage everyone to take a look at our code and we'll listen to your
feedback.
