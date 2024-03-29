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
pub struct ValidatorResponse {
    #[serde(rename = "success")]
    pub success: bool,
}

impl ValidatorResponse {
    pub fn new(success: bool) -> ValidatorResponse {
        ValidatorResponse {
            success,
        }
    }
}


