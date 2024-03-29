/*
 * Thunderstore API
 *
 * Schema is automatically generated and not completely accurate.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionStatus {
    #[serde(rename = "expires")]
    pub expires: String,
}

impl SubscriptionStatus {
    pub fn new(expires: String) -> SubscriptionStatus {
        SubscriptionStatus {
            expires,
        }
    }
}


