use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Controls {
    pub fullscreen: Vec<EventKey>,
    pub reset: Vec<EventKey>,
    pub left: Vec<EventKey>,
    pub right: Vec<EventKey>,
    pub up: Vec<EventKey>,
    pub down: Vec<EventKey>,
    pub shoot: Vec<EventKey>,
    pub transform: Vec<EventKey>,
    pub barrel_dash: Vec<EventKey>,
    pub gas: Vec<EventKey>,
}

impl Controls {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        crate::util::load_file(path).await
    }
}
