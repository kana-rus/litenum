<div align="center">
    <h1>litenum</h1>
</div>

*litenum* is simple convertion utilities between **literal** and **enum** !

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
enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
}

fn main() {
    assert_eq!(
        AnkerTarget::from_lit("_blank").unwrap(),
        AnkerTarget::_blank,
    )
}
```
