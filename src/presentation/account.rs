use crate::presentation::serialization::string_as_float_opt;
use lightstreamer_rs::subscription::ItemUpdate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Representation of account data received from the IG Markets streaming API
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountData {
    /// Name of the item this data belongs to
    item_name: String,
    /// Position of the item in the subscription
    item_pos: i32,
    /// All account fields
    fields: AccountFields,
    /// Fields that have changed in this update
    changed_fields: AccountFields,
    /// Whether this is a snapshot or an update
    is_snapshot: bool,
}

/// Fields containing account financial information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFields {
    #[serde(rename = "PNL")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pnl: Option<f64>,

    #[serde(rename = "DEPOSIT")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    deposit: Option<f64>,

    #[serde(rename = "AVAILABLE_CASH")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    available_cash: Option<f64>,

    #[serde(rename = "PNL_LR")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pnl_lr: Option<f64>,

    #[serde(rename = "PNL_NLR")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pnl_nlr: Option<f64>,

    #[serde(rename = "FUNDS")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    funds: Option<f64>,

    #[serde(rename = "MARGIN")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    margin: Option<f64>,

    #[serde(rename = "MARGIN_LR")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    margin_lr: Option<f64>,

    #[serde(rename = "MARGIN_NLR")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    margin_nlr: Option<f64>,

    #[serde(rename = "AVAILABLE_TO_DEAL")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    available_to_deal: Option<f64>,

    #[serde(rename = "EQUITY")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    equity: Option<f64>,

    #[serde(rename = "EQUITY_USED")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    equity_used: Option<f64>,
}

impl AccountData {
    /// Converts an ItemUpdate from the Lightstreamer API to an AccountData object
    ///
    /// # Arguments
    /// * `item_update` - The ItemUpdate received from the Lightstreamer API
    ///
    /// # Returns
    /// * `Result<Self, String>` - The converted AccountData or an error message
    pub fn from_item_update(item_update: &ItemUpdate) -> Result<Self, String> {
        // Extract the item_name, defaulting to an empty string if None
        let item_name = item_update.item_name.clone().unwrap_or_default();

        // Convert item_pos from usize to i32
        let item_pos = item_update.item_pos as i32;

        // Extract is_snapshot
        let is_snapshot = item_update.is_snapshot;

        // Convert fields
        let fields = Self::create_account_fields(&item_update.fields)?;

        // Convert changed_fields by first creating a HashMap<String, Option<String>>
        let mut changed_fields_map: HashMap<String, Option<String>> = HashMap::new();
        for (key, value) in &item_update.changed_fields {
            changed_fields_map.insert(key.clone(), Some(value.clone()));
        }
        let changed_fields = Self::create_account_fields(&changed_fields_map)?;

        Ok(AccountData {
            item_name,
            item_pos,
            fields,
            changed_fields,
            is_snapshot,
        })
    }

    /// Helper method to create AccountFields from a HashMap of field values
    ///
    /// # Arguments
    /// * `fields_map` - HashMap containing field names and their string values
    ///
    /// # Returns
    /// * `Result<AccountFields, String>` - The parsed AccountFields or an error message
    fn create_account_fields(
        fields_map: &HashMap<String, Option<String>>,
    ) -> Result<AccountFields, String> {
        // Helper function to safely get a field value
        let get_field = |key: &str| -> Option<String> { fields_map.get(key).cloned().flatten() };

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

        Ok(AccountFields {
            pnl: parse_float("PNL")?,
            deposit: parse_float("DEPOSIT")?,
            available_cash: parse_float("AVAILABLE_CASH")?,
            pnl_lr: parse_float("PNL_LR")?,
            pnl_nlr: parse_float("PNL_NLR")?,
            funds: parse_float("FUNDS")?,
            margin: parse_float("MARGIN")?,
            margin_lr: parse_float("MARGIN_LR")?,
            margin_nlr: parse_float("MARGIN_NLR")?,
            available_to_deal: parse_float("AVAILABLE_TO_DEAL")?,
            equity: parse_float("EQUITY")?,
            equity_used: parse_float("EQUITY_USED")?,
        })
    }
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl From<&ItemUpdate> for AccountData {
    fn from(item_update: &ItemUpdate) -> Self {
        Self::from_item_update(item_update).unwrap_or_else(|_| AccountData::default())
    }
}
