use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigExtensionInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigExtensionInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyString,

    #[serde(rename = "cea")]
    pub config_extension_authority: PubkeyString,

    #[serde(rename = "tba")]
    pub token_badge_authority: PubkeyString,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigExtensionInitializedEventOrigin {
    #[serde(rename = "ice")]
    InitializeConfigExtension,
}
