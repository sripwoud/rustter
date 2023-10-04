use nutype::nutype;

#[nutype(validate(min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Username(String);

#[nutype(validate(min_len = 8))]
#[derive(AsRef, Clone, Deserialize, PartialEq, Serialize)]
pub struct Password(String);
