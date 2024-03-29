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
pub struct PackageCategoryExperimental {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl PackageCategoryExperimental {
    pub fn new(name: String, slug: String) -> PackageCategoryExperimental {
        PackageCategoryExperimental {
            name,
            slug,
        }
    }
}


