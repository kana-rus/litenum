<div align="center">
    <h1>litenum</h1>
</div>

*litenum* is minimal utilities for convertion between **literal** and **enum** !


## Features

- *minimal inplementation*
- *no std*, *no alloc*


## How to use

### to literal

```rust
#[litenum::to]
enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
}

fn main() {
    assert_eq!(
        AnkerTarget::_blank.to_lit(),
        "_blank",
    )
}
```

### from literal

```rust
#[litenum::from]
#[derive(Debug, PartialEq)]
enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
}

fn main() {
    assert_eq!(
        AnkerTarget::from_lit("_blank"),
        Some(AnkerTarget::_blank),
    )
}
```

### impl both at once

```rust
#[litenum::ium]  // equals to
                 // `#[litenum::to] #[litenum::from]`
#[derive(Debug, PartialEq)]
enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
}

fn main() {
    assert_eq!(
        AnkerTarget::_blank.to_lit(),
        "_blank",
    );

    assert_eq!(
        AnkerTarget::from_lit("_blank").unwrap(),
        Some(AnkerTarget::_blank),
    );
}
```
