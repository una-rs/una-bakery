# una-bakery — parsing Unicode data and baking normalization tables

[Русский](README.ru.md) | English | [中文](README.zh.md)

----

This crate is intended for parsing Unicode Standard data (current version — 17.0.0) and generating optimized data tables used in the normalization algorithm.

The library is auxiliary; the prepared data is used in the normalization crate [una-normalization](https://github.com/una-rs/una-normalization).

## Usage

```
cargo run
```

As a result, the following will be created:

- Normalization tables in `output/tables`.
- Statistics on code point decomposition/composition features in `output/stats`.

## Trust & Verification

This crate generates tables directly from the official Unicode 17.0.0 data files. No hardcoded tables. No hidden logic.

**To verify:**

- Download source files listed in [input/NOTES.md](input/NOTES.md).
- Run `cargo run`.
- Compare generated hashes with release artifacts.

## Structure

*Main modules:*

- `src/bin` — table generation.
- `src/data` — parsing data from files provided by Unicode.
- `src/normalization` — encoding code point normalization data into the table format.

*Other modules:*

- `src/codepoint` — a code point class with its properties; the data is obtained from `UCD/UnicodeData.txt`.
- `src/hangul` — Hangul, a special case for code points.
- `src/errors` — errors.
- `src/tests` — tests and optional assertion checks that may be useful for those who want to better understand the code and the standard.

**Input and output data**:

- `input` — source Unicode data.
- `output/tables` — generated baked normalization tables.
- `output/stats` — statistics produced during encoding, additional information.
