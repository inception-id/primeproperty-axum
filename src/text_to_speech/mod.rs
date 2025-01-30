mod routes;
mod services;
mod storage;

pub use routes::tts_routes;

pub use services::TextToSpeech;
pub use storage::TextToSpeechStorage;