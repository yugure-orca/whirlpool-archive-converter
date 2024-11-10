use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionBundleInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionBundleInitializedEventOrigin,

    #[serde(rename = "pb")]
    pub position_bundle: PubkeyString,

    #[serde(rename = "pbm")]
    pub position_bundle_mint: PubkeyString,

    #[serde(rename = "pbo")]
    pub position_bundle_owner: PubkeyString,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionBundleInitializedEventOrigin {
    #[serde(rename = "ipb")]
    InitializePositionBundle,
    #[serde(rename = "ipbwm")]
    InitializePositionBundleWithMetadata,
}
