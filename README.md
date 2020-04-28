# uwuizer
Rust Macro to UwUize your text. UwU.

Inspired by:
* [owo text generator](https://honk.moe/tools/owo.html)

## Usage
Add this to ``Cargo.toml``:

```toml
[dependencies]
uwuizer = "0.2.1"
```
``example.rs`` :
```rust
use uwuizer::*;

fn main() {
    let text = uwuize!("euthanize me senpai!!");
    println!("{}", text);
}
```

## Documentation
https://docs.rs/uwuizer/0.2.1/uwuizer/