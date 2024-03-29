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
pub struct UserProfile {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "capabilities")]
    pub capabilities: Vec<String>,
    #[serde(rename = "connections")]
    pub connections: Vec<crate::models::SocialAuthConnection>,
    #[serde(rename = "subscription")]
    pub subscription: Box<crate::models::SubscriptionStatus>,
    #[serde(rename = "rated_packages")]
    pub rated_packages: Vec<String>,
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
    #[serde(rename = "teams_full")]
    pub teams_full: Vec<crate::models::UserTeam>,
}

impl UserProfile {
    pub fn new(username: String, capabilities: Vec<String>, connections: Vec<crate::models::SocialAuthConnection>, subscription: crate::models::SubscriptionStatus, rated_packages: Vec<String>, teams: Vec<String>, teams_full: Vec<crate::models::UserTeam>) -> UserProfile {
        UserProfile {
            username,
            capabilities,
            connections,
            subscription: Box::new(subscription),
            rated_packages,
            teams,
            teams_full,
        }
    }
}


