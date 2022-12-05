# RoboHash
Rust implementation of [RoboHash](https://github.com/e1ven/Robohash/) by [e1ven](https://github.com/e1ven)

## Install
```bash
robohash = "0.1.0"
```

## Example Implementation

```rust
use std::fmt::Error;
use robohash::colour::Colour;
use robohash::RoboHash;
use robohash::set_type::Set;

fn main() -> Result<(), Error> {
    let text = "something_to_turn_into_a_robot";
    let robo = RoboHash::new(text, Set::Default, Colour::Any)?;
    let robo_hash = robo.assemble_base64()?;
    println!("{robo_hash:#?}");
    Ok(())
}
```

## Implemented
- Generate base64 robo hash image from any of the provided sets and colours

## Todo
- [ ] Support backgrounds
- [ ] Support for saving images to disk
- [ ] Support for returning raw image data
- [ ] Support image resizing