pub type PSResult<T> = std::result::Result<T, PSError>;

/// PokeSpeareError. It enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PSError {
    /// Pokemon is not recognized by PokeAPI.
    #[error("Pokemon not found")]
    PokemonNotFound,

    /// PokeAPI does not have an english description for the given pokemon.
    #[error("No english description found for pokemon")]
    NoPokemonEnDescription,

    /// Too much requests. Quota limits reached.
    #[error("Too much requests. Quota limits reached")]
    QuotaError,

    /// Shakespeare translator service does not work as expected.
    #[error("Shakespeare translator error")]
    ShakespeareError,

    /// PokeAPI service does not work as expected.
    #[error("PokeAPI error")]
    PokeApiError,
}
