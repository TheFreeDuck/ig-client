use crate::impl_json_display;
use crate::presentation::serialization::string_as_float_opt;
use lightstreamer_rs::subscription::ItemUpdate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DealingFlag {
    #[serde(rename = "CLOSED")]
    #[default]
    Closed,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "DEAL")]
    Deal,
    #[serde(rename = "EDIT")]
    Edit,
    #[serde(rename = "CLOSINGONLY")]
    ClosingOnly,
    #[serde(rename = "DEALNOEDIT")]
    DealNoEdit,
    #[serde(rename = "AUCTION")]
    Auction,
    #[serde(rename = "AUCTIONNOEDIT")]
    AuctionNoEdit,
    #[serde(rename = "SUSPEND")]
    Suspend,
}

/// Structure for price data received from the IG Markets API
/// Contains information about market prices and related data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceData {
    /// Name of the item (usually the market ID)
    pub item_name: String,
    /// Position of the item in the subscription
    pub item_pos: i32,
    /// All price fields for this market
    pub fields: PriceFields,
    /// Fields that have changed in this update
    pub changed_fields: PriceFields,
    /// Whether this is a snapshot or an update
    pub is_snapshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceFields {
    #[serde(rename = "MID_OPEN")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    mid_open: Option<f64>,

    #[serde(rename = "HIGH")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    high: Option<f64>,

    #[serde(rename = "LOW")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    low: Option<f64>,

    #[serde(rename = "BIDQUOTEID")]
    #[serde(default)]
    bid_quote_id: Option<String>,

    #[serde(rename = "ASKQUOTEID")]
    #[serde(default)]
    ask_quote_id: Option<String>,

    // Bid ladder prices
    #[serde(rename = "BIDPRICE1")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_price1: Option<f64>,

    #[serde(rename = "BIDPRICE2")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_price2: Option<f64>,

    #[serde(rename = "BIDPRICE3")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_price3: Option<f64>,

    #[serde(rename = "BIDPRICE4")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_price4: Option<f64>,

    #[serde(rename = "BIDPRICE5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_price5: Option<f64>,

    // Ask ladder prices
    #[serde(rename = "ASKPRICE1")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_price1: Option<f64>,

    #[serde(rename = "ASKPRICE2")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_price2: Option<f64>,

    #[serde(rename = "ASKPRICE3")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_price3: Option<f64>,

    #[serde(rename = "ASKPRICE4")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_price4: Option<f64>,

    #[serde(rename = "ASKPRICE5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_price5: Option<f64>,

    // Bid sizes
    #[serde(rename = "BIDSIZE1")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_size1: Option<f64>,

    #[serde(rename = "BIDSIZE2")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_size2: Option<f64>,

    #[serde(rename = "BIDSIZE3")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_size3: Option<f64>,

    #[serde(rename = "BIDSIZE4")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_size4: Option<f64>,

    #[serde(rename = "BIDSIZE5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    bid_size5: Option<f64>,

    // Ask sizes
    #[serde(rename = "ASKSIZE1")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_size1: Option<f64>,

    #[serde(rename = "ASKSIZE2")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_size2: Option<f64>,

    #[serde(rename = "ASKSIZE3")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_size3: Option<f64>,

    #[serde(rename = "ASKSIZE4")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_size4: Option<f64>,

    #[serde(rename = "ASKSIZE5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    ask_size5: Option<f64>,

    // Currencies
    #[serde(rename = "CURRENCY0")]
    #[serde(default)]
    currency0: Option<String>,

    #[serde(rename = "CURRENCY1")]
    #[serde(default)]
    currency1: Option<String>,

    #[serde(rename = "CURRENCY2")]
    #[serde(default)]
    currency2: Option<String>,

    #[serde(rename = "CURRENCY3")]
    #[serde(default)]
    currency3: Option<String>,

    #[serde(rename = "CURRENCY4")]
    #[serde(default)]
    currency4: Option<String>,

    #[serde(rename = "CURRENCY5")]
    #[serde(default)]
    currency5: Option<String>,

    // Bid size thresholds
    #[serde(rename = "C1BIDSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c1_bid_size: Option<f64>,

    #[serde(rename = "C2BIDSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c2_bid_size: Option<f64>,

    #[serde(rename = "C3BIDSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c3_bid_size: Option<f64>,

    #[serde(rename = "C4BIDSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c4_bid_size: Option<f64>,

    #[serde(rename = "C5BIDSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c5_bid_size: Option<f64>,

    // Ask size thresholds
    #[serde(rename = "C1ASKSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c1_ask_size: Option<f64>,

    #[serde(rename = "C2ASKSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c2_ask_size: Option<f64>,

    #[serde(rename = "C3ASKSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c3_ask_size: Option<f64>,

    #[serde(rename = "C4ASKSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c4_ask_size: Option<f64>,

    #[serde(rename = "C5ASKSIZE1-5")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    c5_ask_size: Option<f64>,

    #[serde(rename = "TIMESTAMP")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    timestamp: Option<f64>,

    #[serde(rename = "DLG_FLAG")]
    #[serde(default)]
    dealing_flag: Option<DealingFlag>,
}
impl PriceFields {
    pub fn mid_open(&self) -> Option<f64> {
        self.mid_open
    }
    pub fn high(&self) -> Option<f64> {
        self.high
    }
    pub fn low(&self) -> Option<f64> {
        self.low
    }

    pub fn bid_quote_id(&self) -> Option<&str> {
        self.bid_quote_id.as_deref()
    }
    pub fn ask_quote_id(&self) -> Option<&str> {
        self.ask_quote_id.as_deref()
    }

    pub fn bid_price1(&self) -> Option<f64> {
        self.bid_price1
    }
    pub fn bid_price2(&self) -> Option<f64> {
        self.bid_price2
    }
    pub fn bid_price3(&self) -> Option<f64> {
        self.bid_price3
    }
    pub fn bid_price4(&self) -> Option<f64> {
        self.bid_price4
    }
    pub fn bid_price5(&self) -> Option<f64> {
        self.bid_price5
    }

    pub fn ask_price1(&self) -> Option<f64> {
        self.ask_price1
    }
    pub fn ask_price2(&self) -> Option<f64> {
        self.ask_price2
    }
    pub fn ask_price3(&self) -> Option<f64> {
        self.ask_price3
    }
    pub fn ask_price4(&self) -> Option<f64> {
        self.ask_price4
    }
    pub fn ask_price5(&self) -> Option<f64> {
        self.ask_price5
    }

    pub fn bid_size1(&self) -> Option<f64> {
        self.bid_size1
    }
    pub fn bid_size2(&self) -> Option<f64> {
        self.bid_size2
    }
    pub fn bid_size3(&self) -> Option<f64> {
        self.bid_size3
    }
    pub fn bid_size4(&self) -> Option<f64> {
        self.bid_size4
    }
    pub fn bid_size5(&self) -> Option<f64> {
        self.bid_size5
    }

    pub fn ask_size1(&self) -> Option<f64> {
        self.ask_size1
    }
    pub fn ask_size2(&self) -> Option<f64> {
        self.ask_size2
    }
    pub fn ask_size3(&self) -> Option<f64> {
        self.ask_size3
    }
    pub fn ask_size4(&self) -> Option<f64> {
        self.ask_size4
    }
    pub fn ask_size5(&self) -> Option<f64> {
        self.ask_size5
    }

    pub fn currency0(&self) -> Option<&str> {
        self.currency0.as_deref()
    }
    pub fn currency1(&self) -> Option<&str> {
        self.currency1.as_deref()
    }
    pub fn currency2(&self) -> Option<&str> {
        self.currency2.as_deref()
    }
    pub fn currency3(&self) -> Option<&str> {
        self.currency3.as_deref()
    }
    pub fn currency4(&self) -> Option<&str> {
        self.currency4.as_deref()
    }
    pub fn currency5(&self) -> Option<&str> {
        self.currency5.as_deref()
    }

    pub fn c1_bid_size(&self) -> Option<f64> {
        self.c1_bid_size
    }
    pub fn c2_bid_size(&self) -> Option<f64> {
        self.c2_bid_size
    }
    pub fn c3_bid_size(&self) -> Option<f64> {
        self.c3_bid_size
    }
    pub fn c4_bid_size(&self) -> Option<f64> {
        self.c4_bid_size
    }
    pub fn c5_bid_size(&self) -> Option<f64> {
        self.c5_bid_size
    }

    pub fn c1_ask_size(&self) -> Option<f64> {
        self.c1_ask_size
    }
    pub fn c2_ask_size(&self) -> Option<f64> {
        self.c2_ask_size
    }
    pub fn c3_ask_size(&self) -> Option<f64> {
        self.c3_ask_size
    }
    pub fn c4_ask_size(&self) -> Option<f64> {
        self.c4_ask_size
    }
    pub fn c5_ask_size(&self) -> Option<f64> {
        self.c5_ask_size
    }

    pub fn timestamp(&self) -> Option<f64> {
        self.timestamp
    }

    pub fn dealing_flag(&self) -> Option<&DealingFlag> {
        self.dealing_flag.as_ref()
    }
}

impl_json_display!(PriceFields);

impl PriceData {
    /// Converts a Lightstreamer ItemUpdate to a PriceData object
    ///
    /// # Arguments
    ///
    /// * `item_update` - The ItemUpdate from Lightstreamer containing price data
    ///
    /// # Returns
    ///
    /// A Result containing either the parsed PriceData or an error message
    pub fn from_item_update(item_update: &ItemUpdate) -> Result<Self, String> {
        // Extract the item_name, defaulting to an empty string if None
        let item_name = item_update.item_name.clone().unwrap_or_default();

        // Convert item_pos from usize to i32
        let item_pos = item_update.item_pos as i32;

        // Extract is_snapshot
        let is_snapshot = item_update.is_snapshot;

        // Convert fields
        let fields = Self::create_price_fields(&item_update.fields)?;

        // Convert changed_fields by first creating a HashMap<String, Option<String>>
        let mut changed_fields_map: HashMap<String, Option<String>> = HashMap::new();
        for (key, value) in &item_update.changed_fields {
            changed_fields_map.insert(key.clone(), Some(value.clone()));
        }
        let changed_fields = Self::create_price_fields(&changed_fields_map)?;

        Ok(PriceData {
            item_name,
            item_pos,
            fields,
            changed_fields,
            is_snapshot,
        })
    }

    // Helper method to create PriceFields from a HashMap
    fn create_price_fields(
        fields_map: &HashMap<String, Option<String>>,
    ) -> Result<PriceFields, String> {
        // Helper function to safely get a field value
        let get_field = |key: &str| -> Option<String> { fields_map.get(key).cloned().flatten() };

        // Helper function to parse float values
        let parse_float = |key: &str| -> Result<Option<f64>, String> {
            match get_field(key) {
                Some(val) if !val.is_empty() => val
                    .parse::<f64>()
                    .map(Some)
                    .map_err(|_| format!("Failed to parse {key} as float: {val}")),
                _ => Ok(None),
            }
        };

        // Parse dealing flag
        let dealing_flag = match get_field("DLG_FLAG").as_deref() {
            Some("CLOSED") => Some(DealingFlag::Closed),
            Some("CALL") => Some(DealingFlag::Call),
            Some("DEAL") => Some(DealingFlag::Deal),
            Some("EDIT") => Some(DealingFlag::Edit),
            Some("CLOSINGONLY") => Some(DealingFlag::ClosingOnly),
            Some("DEALNOEDIT") => Some(DealingFlag::DealNoEdit),
            Some("AUCTION") => Some(DealingFlag::Auction),
            Some("AUCTIONNOEDIT") => Some(DealingFlag::AuctionNoEdit),
            Some("SUSPEND") => Some(DealingFlag::Suspend),
            Some(unknown) => return Err(format!("Unknown dealing flag: {unknown}")),
            None => None,
        };

        Ok(PriceFields {
            mid_open: parse_float("MID_OPEN")?,
            high: parse_float("HIGH")?,
            low: parse_float("LOW")?,
            bid_quote_id: get_field("BIDQUOTEID"),
            ask_quote_id: get_field("ASKQUOTEID"),

            // Bid ladder prices
            bid_price1: parse_float("BIDPRICE1")?,
            bid_price2: parse_float("BIDPRICE2")?,
            bid_price3: parse_float("BIDPRICE3")?,
            bid_price4: parse_float("BIDPRICE4")?,
            bid_price5: parse_float("BIDPRICE5")?,

            // Ask ladder prices
            ask_price1: parse_float("ASKPRICE1")?,
            ask_price2: parse_float("ASKPRICE2")?,
            ask_price3: parse_float("ASKPRICE3")?,
            ask_price4: parse_float("ASKPRICE4")?,
            ask_price5: parse_float("ASKPRICE5")?,

            // Bid sizes
            bid_size1: parse_float("BIDSIZE1")?,
            bid_size2: parse_float("BIDSIZE2")?,
            bid_size3: parse_float("BIDSIZE3")?,
            bid_size4: parse_float("BIDSIZE4")?,
            bid_size5: parse_float("BIDSIZE5")?,

            // Ask sizes
            ask_size1: parse_float("ASKSIZE1")?,
            ask_size2: parse_float("ASKSIZE2")?,
            ask_size3: parse_float("ASKSIZE3")?,
            ask_size4: parse_float("ASKSIZE4")?,
            ask_size5: parse_float("ASKSIZE5")?,

            // Currencies
            currency0: get_field("CURRENCY0"),
            currency1: get_field("CURRENCY1"),
            currency2: get_field("CURRENCY2"),
            currency3: get_field("CURRENCY3"),
            currency4: get_field("CURRENCY4"),
            currency5: get_field("CURRENCY5"),

            // Bid size thresholds
            c1_bid_size: parse_float("C1BIDSIZE1-5")?,
            c2_bid_size: parse_float("C2BIDSIZE1-5")?,
            c3_bid_size: parse_float("C3BIDSIZE1-5")?,
            c4_bid_size: parse_float("C4BIDSIZE1-5")?,
            c5_bid_size: parse_float("C5BIDSIZE1-5")?,

            // Ask size thresholds
            c1_ask_size: parse_float("C1ASKSIZE1-5")?,
            c2_ask_size: parse_float("C2ASKSIZE1-5")?,
            c3_ask_size: parse_float("C3ASKSIZE1-5")?,
            c4_ask_size: parse_float("C4ASKSIZE1-5")?,
            c5_ask_size: parse_float("C5ASKSIZE1-5")?,

            timestamp: parse_float("TIMESTAMP")?,
            dealing_flag,
        })
    }
}

impl_json_display!(PriceData);

impl From<&ItemUpdate> for PriceData {
    fn from(item_update: &ItemUpdate) -> Self {
        Self::from_item_update(item_update).unwrap_or_default()
    }
}
