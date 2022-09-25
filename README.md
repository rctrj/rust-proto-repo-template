This repository shows how to create a repository to store protos for rust projects that use [Tonic](https://github.com/hyperium/tonic) with [Prost](https://github.com/tokio-rs/prost).

What happens here:
- `build.rs` generates stubs for all proto files present in `pb` folder
- `lib.rs` exports all the protos into a mod

This can then be used as a crate for accessing proto files.
___

## Credits
- `build.rs` is copied from [here](https://gist.github.com/aquarhead/69092f21347353981357909bea7765e4)
All credit to author: [aquarhead](https://gist.github.com/aquarhead)
