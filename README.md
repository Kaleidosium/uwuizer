# uwuizer
Rust Macro to UwUize your text. UwU.

Inspired by:
* [owo text generator](https://honk.moe/tools/owo.html)

# Example
``example.rs`` :
```rust
use uwuizer::*;

fn main() {
    let text = uwuize!("euthanize me senpai!!");
    println!("{}", text);
}
```