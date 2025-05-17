/// Response structure for session-related API calls
#[derive(serde::Deserialize)]
pub struct SessionResp {
    /// Account ID associated with the session
    #[serde(alias = "accountId")]
    #[serde(alias = "currentAccountId")]
    pub account_id: String,

    /// Client ID provided by the API
    #[serde(alias = "clientId")]
    pub client_id: Option<String>,
    /// Timezone offset in hours
    #[serde(alias = "timezoneOffset")]
    pub timezone_offset: Option<i32>,
}
