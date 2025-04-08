use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct QueuedRecording {
    pub(crate) duration: f32,
    pub(crate) url: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FeedUpdateEvent {
    #[allow(non_snake_case)]
    sentence {
        sentenceId: String,
    },
    #[allow(non_snake_case)]
    word {
        wordId: String,
        meaningIdx: i32,
    },
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UIUpdateMessage {
    #[allow(non_snake_case)]
    feed {
        value: FeedUpdateEvent,
        duration: f32,
    },
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueuedTrackItem {
    #[allow(non_snake_case)]
    recording {
        id: i32,
        value: QueuedRecording,
        targetTime: f32,
    },
    #[allow(non_snake_case)]
    message {
        id: i32,
        value: serde_json::Value,
        targetTime: f32,
    },
    #[allow(non_snake_case)]
    ui {
        id: i32,
        value: UIUpdateMessage,
        targetTime: f32,
    },
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum ServerMessage {
    #[allow(non_snake_case)]
    queueTrackItems {
        trackItems: Vec<QueuedTrackItem>,
        iterationToken: i32,
        atEnd: bool,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum ClientMessage {
    #[allow(non_snake_case)]
    requestContent { iterationToken: u32 },
    #[allow(non_snake_case)]
    play {
        streamName: String,
        parameters: serde_json::Value,
        iterationToken: u32,
    },
    #[allow(non_snake_case)]
    message {
        message: serde_json::Value,
    },
}