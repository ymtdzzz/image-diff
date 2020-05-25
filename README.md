## image-diff [![image-diff at crates.io](https://badgen.net/crates/v/image-diff)](https://crates.io/crates/image-diff) [![image-diff at docs.rs](https://docs.rs/image-diff/badge.svg)](https://docs.rs/image-diff/0.1.1/image_diff/)
An image diff library written in rust

![sample](sample.png)

### Definition

``` rust
pub fn diff(before: &DynamicImage, after: &DynamicImage) -> Result<DynamicImage>
```

### Usage

```toml
[dependencies]
image = "0.23.4"
image-diff = "0.1.0"
```

``` rust
let before = image::open("before.png");
let after = image::open("after.png");
let image_diff = diff(&before, &after).unwrap();
image_diff.save("image_diff.png");
```
