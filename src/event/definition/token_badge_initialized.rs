use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct TokenBadgeInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: TokenBadgeInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyString,

    #[serde(rename = "tm")]
    pub token_mint: PubkeyString,

    #[serde(rename = "tb")]
    pub token_badge: PubkeyString,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum TokenBadgeInitializedEventOrigin {
    #[serde(rename = "itb")]
    InitializeTokenBadge,
}
