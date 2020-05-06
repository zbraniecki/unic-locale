# Changelog

## Unreleased

  - …

## unic-locale 0.9.0 (May 6, 2020)

 - Separate out subtags into their own structs.
 - Update to CLDR 37 full.
 - Add serde derives when `serde` feature is on.
 - Remove deprecated Error descriptions.

## unic-locale 0.8.0 (January 28, 2020)

 - Switch likelysubtags method names to maximize/minimize.
 - Implement RFC 344 by removing get prefixes.
 - Use `sort_unstable`.

## unic-locale 0.7.1 (November 10, 2019)

  - Add `PartialOrd` and Ord` for `Locale`.

## unic-locale 0.7.0 (October 29, 2019)

  - Separate out `clear_*` methods.
  - Switch the API to operate on `AsRef<[u8]>`.
  - Switch set-based methods to use iterators.
  - Update TinyStr to 0.3.2.
  - Add extension managing methods (#30).
  - Add documentation.

## unic-locale 0.6.0 (October 3, 2019)

  - Add `add_likely_subtags` and `remove_likely_subtags`.
  - Add `get_character_direction`.

## unic-locale 0.5.0 (September 5, 2019)

  - Complete well-formed logic.

## unic-locale 0.4.2 (August 2, 2019)

  - Update the macros to 0.3.0.

## unic-locale 0.4.1 (July 29, 2019)

  - Update the macros to 0.2.0 to make the macro work without explicit import of the impl.

## unic-locale 0.4.0 (July 26, 2019)

  - Switch to FromStr instead of TryFrom
  - Add locale! macro
  - Skip parsing and allocating when using macros

## unic-locale 0.3.0 (July 24, 2019)

  - Switch variants to handle Option for ergonomics
  - Extend fixtures coverage
  - Introduce From/Into/AsRef helpers

## unic-locale 0.2.0 (July 9, 2019)

  - Expose LanguageIdentifier from unic_locale
  - Switch to TryFrom
  - Improve Extension handling

## unic-locale 0.1.0 (June 15, 2019)

  - Initial release
