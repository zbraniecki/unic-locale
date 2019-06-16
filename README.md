[![Build Status](https://travis-ci.org/zbraniecki/unic-locale.svg?branch=master)](https://travis-ci.org/zbraniecki/unic-locale) [![Coverage Status](https://coveralls.io/repos/github/zbraniecki/unic-locale/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/unic-locale?branch=master)

# unic-langid [![crates.io](http://meritbadge.herokuapp.com/unic-langid)](https://crates.io/crates/unic-langid)

Unic crate for Unicode Language Identifier management.

# unic-locale [![crates.io](http://meritbadge.herokuapp.com/funic-locale)](https://crates.io/crates/unic-locale)

Unic crates for Unicode Locale management.

# Unicode Conformance

All code implements (parts) of [Unicode UTS #35 Language and Locale Identifiers](http://unicode.org/reports/tr35/#Identifiers).

The API is intended to function similarly to [ICU Locale](http://icu-project.org/apiref/icu4c/classicu_1_1Locale.html) and [ECMA402 Intl.Locale](https://github.com/tc39/proposal-intl-locale/).

# Performance

In early tests the performance of parsing a set of language identifiers into a well formed struct, `unic-langid` is [~50 times faster](https://gist.github.com/zbraniecki/016f7bd35fc6e09aede997c5bc20222a) than ICU Locale.
