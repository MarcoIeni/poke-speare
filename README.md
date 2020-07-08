# Poke-speare

REST API that, given a Pokémon name, returns its description in Shakespeare's
style.

Pokemon description is taken from [PokéAPI](https://pokeapi.co/) and it is converted
by using [Shakespeare translator](https://funtranslations.com/api/shakespeare).

## Usage

You can use `poke-speare` both as a cli program or as a rust library

### Cli

```sh
$ ./poke-speare &
$ curl http://localhost:5000/pokemon/charizard

{
    "name": "charizard",
    "description": "Charizard flies 'round the sky in search of powerful opponents."
}

```

### Library

TODO

## Install

### Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install poke-speare`

### Docker

TODO

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
