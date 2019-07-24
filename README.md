[![Build Status](https://travis-ci.org/zbraniecki/unic-locale.svg?branch=master)](https://travis-ci.org/zbraniecki/unic-locale) [![Coverage Status](https://coveralls.io/repos/github/zbraniecki/unic-locale/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/unic-locale?branch=master)

This meta-crate contains two basic crates for Unicode Language Identifier and Locale manipulation.

# unic-langid [![crates.io](http://meritbadge.herokuapp.com/unic-langid)](https://crates.io/crates/unic-langid)

Unic crate for [Unicode Language Identifiers](http://unicode.org/reports/tr35/#Unicode_language_identifier) management.

Language Identifier describes a combination of language, region, script and variants. Examples: `"en-US"`, `"sr-Cyrl-RU"`, `"de-AT"`, `"zh-Hans"`.

# unic-locale [![crates.io](http://meritbadge.herokuapp.com/unic-locale)](https://crates.io/crates/unic-locale)

Unic crates for [Unicode Locale Identifiers](http://unicode.org/reports/tr35/#Unicode_locale_identifier) management.

Locale Identifiers extend Language Identifiers with a set of extensions for *unicode*, *transform* and *private*.

This allows the user to encode additional data. Examples: `"en-US-u-hc-h24"`, `"pl-u-ca-buddhist"`.

# Status

The `unic-langid` crate is fully functional and should parse/manipulate/serialize in conformance with the standard.
The Rust API is still rough and will change as we progress toward 1.0, and performance can be improved.

The `unic-locale` crate is incomplete with only several examples of extensions handled at the moment. The goal is to advance the coverage as the API progresses.
The API shadows that of `unic-langid` and is intended to be a drop-in replacement for `unic-langid` so that users can start with a simple Language Identifier coverage and extend it to use `unic-locale` once needed.

# Unicode Conformance

All code implements (parts) of [Unicode UTS #35 Language and Locale Identifiers](http://unicode.org/reports/tr35/#Identifiers).

The API is intended to function similarly to [ICU Locale](http://icu-project.org/apiref/icu4c/classicu_1_1Locale.html) and [ECMA402 Intl.Locale](https://github.com/tc39/proposal-intl-locale/).

# Performance

In early tests the performance of parsing a set of language identifiers into a well formed struct, `unic-langid` is [~50 times faster](https://gist.github.com/zbraniecki/016f7bd35fc6e09aede997c5bc20222a) than ICU Locale.

There's a lot of room for improvement since at the moment the crates use `String` fields which can be replaced with variants tailored for short strings, and the parser uses naive parsing, which can also be optimized later.