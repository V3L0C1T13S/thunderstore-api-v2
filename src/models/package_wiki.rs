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
pub struct PackageWiki {
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "wiki")]
    pub wiki: Box<crate::models::Wiki>,
}

impl PackageWiki {
    pub fn new(namespace: String, name: String, wiki: crate::models::Wiki) -> PackageWiki {
        PackageWiki {
            namespace,
            name,
            wiki: Box::new(wiki),
        }
    }
}


