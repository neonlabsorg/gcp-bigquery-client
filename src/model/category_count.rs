//! Represents the count of a single category within the cluster.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCount {
    /// The count of training samples matching the category within the cluster.
    pub count: Option<i64>,
    /// The name of category.
    pub category: Option<String>,
}