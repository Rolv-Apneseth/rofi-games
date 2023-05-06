# Changelog

## 1.0.2 (2023-05-06)

- Attempt #2 to get `Makefile` to work for `PKGBUILD`

## 1.0.1 (2023-05-06)

- Attempt to adjust `Makefile` so that it works for `PKGBUILD`

## 1.0.0 (2023-05-06)

### Breaking Changes

- No longer does anything with desktop files, and will just send `rofi` data directly. This has the added benefit of not cluttering up your `drun` menu, and was a big oversight on my part due to the fact that I never use the `drun` menu
- No longer ships a binary, but is a library to be used as a `rofi` plugin instead. This should have been the approach from the start I just didn't know there was a rust wrapper

### Fixes

- Get environment variable for `$HOME` at runtime by using `std::env::var`, instead of using the `env!()` marco which would only get it at build time. Hopefully this wasn't causing issues for anyone though.
