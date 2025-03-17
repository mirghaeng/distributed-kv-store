use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreGetResponse {

    /// Indicates whether the key exists in the store
    pub does_exist: bool,

    /// Optional error message if request fails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Optional message about the request result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The value associated with the key (if it exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
