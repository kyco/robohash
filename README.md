# RoboHash

Rust implementation of [RoboHash](https://github.com/e1ven/Robohash/) by [e1ven](https://github.com/e1ven)

## Install

```bash
robohash = "0.2.2"
```

## Example Implementation

```rust
use std::fmt::Error;
use robohash::RoboHashBuilder;

fn main() -> Result<(), Error> {
    let text = "test";
    let robo = RoboHashBuilder::new(text).build();
    let robo_hash = robo.assemble_base64()?;
    println!("{robo_hash:#?}");
    Ok(())
}
````

### Define Size

```rust
let width = 512;
let height = 512;
let robo = RoboHashBuilder::new("test")
.with_size(width, height)
.build();
```

### Define Colour

```rust
let robo = RoboHashBuilder::new("test")
.with_colour(Colour::Green)
.build();
```

### Define Set

```rust
let robo = RoboHashBuilder::new("test")
.with_set(Set::Set3)
.build();
```

### Change Sets Directory

```rust
let robo = RoboHashBuilder::new("test")
.with_set_location("./sets_location")
.build();
```

### Define Background

```rust
let robo = RoboHashBuilder::new("test")
.with_background_set("bg1")
.build();
```

### Change Background Directory

```rust
let robo = RoboHashBuilder::new("test")
.with_background_location("./backgrounds")
.build();
```

### Full Example

```rust
use std::fmt::Error;
use robohash::RoboHashBuilder;

fn main() -> Result<(), Error> {
    let text = "test";
    let robo = RoboHashBuilder::new(text)
        .with_set(Set::Set1)
        .with_colour(Colour::Green)
        .with_set_location("./sets-root")
        .with_background_set("bg1")
        .with_background_location("./backgrounds")
        .with_size(512, 512)
        .build();
    let robo_hash = robo.assemble_base64()?;
    println!("{robo_hash:#?}");
    Ok(())
}
```

## Implemented

- Generate base64 robo hash image from any of the provided sets and colours
- Background support
- Image size support

## Todo

- [x] Support backgrounds
- [ ] Support for saving images to disk
- [ ] Support for returning raw image data
- [x] Support image sizing
