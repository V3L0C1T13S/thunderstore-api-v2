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
pub struct CompletedPart {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "PartNumber")]
    pub part_number: i32,
}

impl CompletedPart {
    pub fn new(e_tag: String, part_number: i32) -> CompletedPart {
        CompletedPart {
            e_tag,
            part_number,
        }
    }
}


