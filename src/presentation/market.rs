use crate::presentation::serialization::{string_as_bool_opt, string_as_float_opt};
use lightstreamer_rs::subscription::ItemUpdate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents the current state of a market
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketState {
    Closed,
    #[default]
    Offline,
    Tradeable,
    Edit,
    Auction,
    AuctionNoEdit,
    Suspended,
}

/// Representation of market data received from the IG Markets streaming API
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketData {
    /// Name of the item this data belongs to
    item_name: String,
    /// Position of the item in the subscription
    item_pos: i32,
    /// All market fields
    fields: MarketFields,
    /// Fields that have changed in this update
    changed_fields: MarketFields,
    /// Whether this is a snapshot or an update
    is_snapshot: bool,
}

impl MarketData {
    /// Converts an ItemUpdate from the Lightstreamer API to a MarketData object
    ///
    /// # Arguments
    /// * `item_update` - The ItemUpdate received from the Lightstreamer API
    ///
    /// # Returns
    /// * `Result<Self, String>` - The converted MarketData or an error message
    pub fn from_item_update(item_update: &ItemUpdate) -> Result<Self, String> {
        // Extract the item_name, defaulting to an empty string if None
        let item_name = item_update.item_name.clone().unwrap_or_default();

        // Convert item_pos from usize to i32
        let item_pos = item_update.item_pos as i32;

        // Extract is_snapshot
        let is_snapshot = item_update.is_snapshot;

        // Convert fields
        let fields = Self::create_market_fields(&item_update.fields)?;

        // Convert changed_fields by first creating a HashMap<String, Option<String>>
        let mut changed_fields_map: HashMap<String, Option<String>> = HashMap::new();
        for (key, value) in &item_update.changed_fields {
            changed_fields_map.insert(key.clone(), Some(value.clone()));
        }
        let changed_fields = Self::create_market_fields(&changed_fields_map)?;

        Ok(MarketData {
            item_name,
            item_pos,
            fields,
            changed_fields,
            is_snapshot,
        })
    }

    /// Helper method to create MarketFields from a HashMap of field values
    ///
    /// # Arguments
    /// * `fields_map` - HashMap containing field names and their string values
    ///
    /// # Returns
    /// * `Result<MarketFields, String>` - The parsed MarketFields or an error message
    fn create_market_fields(
        fields_map: &HashMap<String, Option<String>>,
    ) -> Result<MarketFields, String> {
        // Helper function to safely get a field value
        let get_field = |key: &str| -> Option<String> { fields_map.get(key).cloned().flatten() };

        // Parse market state
        let market_state = match get_field("MARKET_STATE").as_deref() {
            Some("closed") => Some(MarketState::Closed),
            Some("offline") => Some(MarketState::Offline),
            Some("tradeable") => Some(MarketState::Tradeable),
            Some("edit") => Some(MarketState::Edit),
            Some("auction") => Some(MarketState::Auction),
            Some("auction_no_edit") => Some(MarketState::AuctionNoEdit),
            Some("suspended") => Some(MarketState::Suspended),
            Some(unknown) => return Err(format!("Unknown market state: {}", unknown)),
            None => None,
        };

        // Parse boolean field
        let market_delay = match get_field("MARKET_DELAY").as_deref() {
            Some("0") => Some(false),
            Some("1") => Some(true),
            Some(val) => return Err(format!("Invalid MARKET_DELAY value: {}", val)),
            None => None,
        };

        // Helper function to parse float values
        let parse_float = |key: &str| -> Result<Option<f64>, String> {
            match get_field(key) {
                Some(val) if !val.is_empty() => val
                    .parse::<f64>()
                    .map(Some)
                    .map_err(|_| format!("Failed to parse {} as float: {}", key, val)),
                _ => Ok(None),
            }
        };

        Ok(MarketFields {
            mid_open: parse_float("MID_OPEN")?,
            high: parse_float("HIGH")?,
            offer: parse_float("OFFER")?,
            change: parse_float("CHANGE")?,
            market_delay,
            low: parse_float("LOW")?,
            bid: parse_float("BID")?,
            change_pct: parse_float("CHANGE_PCT")?,
            market_state,
            update_time: get_field("UPDATE_TIME"),
        })
    }
}

impl fmt::Display for MarketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl From<&ItemUpdate> for MarketData {
    fn from(item_update: &ItemUpdate) -> Self {
        Self::from_item_update(item_update).unwrap_or_else(|_| MarketData {
            item_name: String::new(),
            item_pos: 0,
            fields: MarketFields::default(),
            changed_fields: MarketFields::default(),
            is_snapshot: false,
        })
    }
}

/// Fields containing market price and status information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketFields {
    #[serde(rename = "MID_OPEN")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    mid_open: Option<f64>,

    #[serde(rename = "HIGH")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    high: Option<f64>,

    #[serde(rename = "OFFER")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    offer: Option<f64>,

    #[serde(rename = "CHANGE")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    change: Option<f64>,

    #[serde(rename = "MARKET_DELAY")]
    #[serde(with = "string_as_bool_opt")]
    #[serde(default)]
    market_delay: Option<bool>,

    #[serde(rename = "LOW")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    low: Option<f64>,

    #[serde(rename = "BID")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid: Option<f64>,

    #[serde(rename = "CHANGE_PCT")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    change_pct: Option<f64>,

    #[serde(rename = "MARKET_STATE")]
    #[serde(default)]
    market_state: Option<MarketState>,

    #[serde(rename = "UPDATE_TIME")]
    #[serde(default)]
    update_time: Option<String>,
}
