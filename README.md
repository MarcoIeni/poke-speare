# Poke-speare

[![CI](https://github.com/MarcoIeni/poke-speare/workflows/General/badge.svg)](https://github.com/poetry-book/poetry-book-cli/actions)
[![Coverage Status](https://coveralls.io/repos/github/MarcoIeni/poke-speare/badge.svg?branch=master)](https://coveralls.io/github/MarcoIeni/poke-speare?branch=master)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

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

## Credits

- github actions are taken from [LukeMathWalker](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3).
