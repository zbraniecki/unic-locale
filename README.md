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
The Rust API is considered close to stable.

The `unic-locale` crate is mostly complete with full support for unicode, transform and private extensions. The `other` extension type is not currently supported.
The API shadows that of `unic-langid` and is intended to be a drop-in replacement for `unic-langid` so that users can start with a simple Language Identifier coverage and extend it to use `unic-locale` once needed.

# Unicode Conformance

All code implements (parts) of [Unicode UTS #35 Language and Locale Identifiers](http://unicode.org/reports/tr35/#Identifiers).

The API is intended to function similarly to [ICU Locale](http://icu-project.org/apiref/icu4c/classicu_1_1Locale.html) and [ECMA402 Intl.Locale](https://github.com/tc39/proposal-intl-locale/).

# Performance

In early tests the performance of parsing a set of language identifiers into a well formed struct, `unic-langid` is [~50 times faster](https://gist.github.com/zbraniecki/016f7bd35fc6e09aede997c5bc20222a) than ICU Locale.

Performance is consider close to optimal at the moment.

# Meta-crate

Whoa, I can see you saying, what is it about with all those crates here?

Well, that's an excellent question!

Unless you want a deep dive into procedural macro and [proc-macro-hack](https://github.com/dtolnay/proc-macro-hack), let me just give you a tl;dr:

 - `unic-{langid|locale}` - User facing public crates that you should use. Potentially with `features = ["macros"]`
 - `unic-{langid|locale|-impl` - The real code behind all of this.
 - `unic-{langid|locale}-macro-impl` - Actual implementations of the procedural macros
 - `unic-{langid|locale}-macro` - Declaration crates for the macros

In result, there's a little bit of hackery here to get everything work nicely for you, but all you should care about are the two top crates with optional features if you want.

The rest will hopefully go away one day once we mature the macros ecosystem in Rust.
