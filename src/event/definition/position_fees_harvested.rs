use super::{PubkeyString, TransferInfo};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionFeesHarvestedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionFeesHarvestedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,

    // transfer info
    #[serde(rename = "ta")]
    pub transfer_a: TransferInfo,
    #[serde(rename = "tb")]
    pub transfer_b: TransferInfo,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionFeesHarvestedEventOrigin {
    #[serde(rename = "cf")]
    CollectFees,
    #[serde(rename = "cfv2")]
    CollectFeesV2,
}