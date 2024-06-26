#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchChannelJson {
    pub code: usize,
    pub message: String,
    pub content: Content,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Content {
    pub size: usize,
    pub data: Vec<Data>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Data {
    pub channel: Channel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Channel {
    pub channelId: String,
    pub channelName: String,
    pub channelImageUrl: String,
    pub verifiedMark: bool,
    pub channelDescription: String,
    pub followerCount: usize,
    pub openLive: bool,
}

impl crate::search::SearchInfo for SearchChannelJson {}
