use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApplicationInformation {
    #[serde(rename = "modificationTokens")]
    pub modification_tokens: bool,
    #[serde(rename = "pasteLifetime")]
    pub paste_lifetime: isize,
    pub reports: bool,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PfEncryption {
    pub alg: String,
    pub iv: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub pf_encryption: Option<PfEncryption>,
}

#[derive(Deserialize, Debug)]
pub struct Paste {
    pub id: String,
    pub content: String,
    pub created: usize,
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Debug)]
pub struct CreatePasteRequest {
    pub content: String,
    pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Debug)]
pub struct CreatedPaste {
    #[serde(rename = "modificationToken")]
    pub modification_token: String,
    #[serde(flatten)]
    pub paste: Paste,
}
