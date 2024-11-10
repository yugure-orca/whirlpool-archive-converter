use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct TokenBadgeDeletedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: TokenBadgeDeletedEventOrigin,

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
pub enum TokenBadgeDeletedEventOrigin {
    #[serde(rename = "dtb")]
    DeleteTokenBadge,
}
