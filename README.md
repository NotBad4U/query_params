[![Build Status](https://travis-ci.org/NotBad4U/query_params.svg?branch=master)](https://travis-ci.org/NotBad4U/query_params)
[![Rust version]( https://img.shields.io/badge/rust-stable-blue.svg)]()


# QueryParams Derive

[Rust][rust] custom derive to automatically implement serialization to http query params for arbitrary structs. A simple `#[derive(QueryParams)]` will generate a function `to_query_params` for your struct.

## How it Works

```rust
#[macro_use]
extern crate query_params;

#[derive(QueryParams)]
struct PullRequestsParametersApi {
    page: i32,
    sort: bool,
    direction: String,
    state: Vec<String>,
    // .. other interesting fields ..
} 

fn main() {
    let pr = PullRequestsParametersApi {
        page: 2,
        sort: true,
        direction: "asc",
        state: vec!["open".to_string(), "closed".to_string()],
    }

    println!("{}", pr.to_query_params()); // => ?page=2&sort=true&direction=asc&state=open,closed
}
```

## Get Started

It's as simple as two steps:

1. Add `query_params` to your `Cargo.toml` 
  * manually 

  * or with [cargo-edit](https://github.com/killercup/cargo-edit):
  
    `cargo add derive_builder`

2. Annotate your struct with `#[derive(QueryParams)]`

## Disclaimer :exclamation:

* Tuple structs and unit structs are not supported as they have no field names.

## [Documentation][doc]

Detailed explaination of all features and tips for troubleshooting.

### Contribution

Feel free to make a pull request :smiley:

[doc]: https://docs.rs/query_params
[rust]: https://www.rust-lang.org/