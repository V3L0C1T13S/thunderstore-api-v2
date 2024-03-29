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
pub struct OwLoginResponseBody {
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::UserProfile>,
}

impl OwLoginResponseBody {
    pub fn new(session_id: String, profile: crate::models::UserProfile) -> OwLoginResponseBody {
        OwLoginResponseBody {
            session_id,
            profile: Box::new(profile),
        }
    }
}


