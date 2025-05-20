use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    /// Binary options
    Binary,
    /// Bungee capped instruments
    BungeeCapped,
    /// Bungee commodities instruments
    BungeeCommodities,
    /// Bungee currencies instruments
    BungeeCurrencies,
    /// Bungee indices instruments
    BungeeIndices,
    /// Commodities instruments
    Commodities,
    /// Currency pairs
    Currencies,
    /// Market indices
    Indices,
    /// Knockouts commodities instruments
    KnockoutsCommodities,
    /// Knockouts currencies instruments
    KnockoutsCurrencies,
    /// Knockouts indices instruments
    KnockoutsIndices,
    /// Knockouts shares instruments
    KnockoutsShares,
    /// Options on commodities
    OptCommodities,
    /// Options on currencies
    OptCurrencies,
    /// Options on indices
    OptIndices,
    /// Options on rates
    OptRates,
    /// Options on shares
    OptShares,
    /// Interest rates
    Rates,
    /// Market sectors
    Sectors,
    /// Stocks and shares
    Shares,
    /// Sprint market instruments
    SprintMarket,
    /// Test market instruments
    TestMarket,
    /// Unknown instrument type
    Unknown,
    /// Options
    Options,
}
