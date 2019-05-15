# unic-locale
Unic crates for locale management

All code implements (parts) of [Unicode UTS #35 Language and Locale Identifiers](http://unicode.org/reports/tr35/#Identifiers).

And is intended to function similarly to [ICU Locale](http://icu-project.org/apiref/icu4c/classicu_1_1Locale.html) and [ECMA402 Intl.Locale](https://github.com/tc39/proposal-intl-locale/).

# Performance

In early tests the performance of parsing a set of language identifiers into a well formed struct, `unic-langid` is [~50 times faster](https://gist.github.com/zbraniecki/016f7bd35fc6e09aede997c5bc20222a) than ICU Locale.
