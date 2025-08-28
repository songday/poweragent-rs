use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum EmbeddingModel {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum CompletionsModel {
    AllMiniLML6V2,
    ParaphraseMLMiniLML12V2,
    ParaphraseMLMpnetBaseV2,
    BgeSmallEnV1_5,
    BgeBaseEnV1_5,
    BgeLargeEnV1_5,
    BgeM3,
    NomicEmbedTextV1_5,
    MultilingualE5Small,
    MultilingualE5Base,
    MultilingualE5Large,
    MxbaiEmbedLargeV1,
    Phi3Mini4kInstruct,
    TinyLlama1_1bChatV1_0,
    Gemma2bInstruct,
    Gemma7bInstruct,
    ParlerTtsMiniV1,
    ParlerTtsLargeV1,
    WhisperLargeV3,
}