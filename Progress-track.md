# Progress track

> temporary file, will be deleted before first PR

## binary size and opts (release)

### SIZE (linux)

- Initial size => 13mb !!
- After strip => 9.1 mb
- lto => 8.2 mb
- codegen => 8.2 mb

```x

crates bloat:

 File  .text     Size Crate
12.8%  25.0%   1.5MiB rumdl
 7.4%  14.3% 897.6KiB std
 3.8%   7.5% 467.4KiB regex_automata
 3.8%   7.4% 460.7KiB tower_lsp
 3.0%   5.9% 372.0KiB serde
 2.5%   4.9% 304.3KiB clap_builder
 1.7%   3.3% 205.9KiB regex_syntax
 1.7%   3.3% 204.9KiB aho_corasick
 1.7%   3.2% 202.3KiB markdown
 1.4%   2.7% 170.3KiB toml_edit
 1.4%   2.6% 165.6KiB tokio
 1.2%   2.3% 143.3KiB serde_json
 1.0%   2.0% 124.5KiB futures_util
 1.0%   2.0% 122.3KiB lsp_types
 0.8%   1.5%  95.6KiB ignore
 0.8%   1.5%  93.5KiB fancy_regex
 0.6%   1.2%  76.6KiB globset
 0.6%   1.2%  74.6KiB tower
 0.4%   0.8%  48.4KiB url
 0.4%   0.8%  47.9KiB unsafe_libyaml
 2.9%   5.7% 356.4KiB And 51 more crates. Use -n N to show more.
51.3% 100.0%   6.1MiB .text section size,

```

## performance

no unit-bench for now, just compare check + fix of [this file]
for reference, same has been done using markdownlint-cli2
