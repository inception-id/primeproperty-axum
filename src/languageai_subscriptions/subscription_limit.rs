use crate::checkbot::{Checkbot, CheckbotStorage};
use crate::db::DbPool;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::languageai_subscriptions::LanguageaiSubscription;
use crate::speech_to_text::{SpeechToText, SpeechToTextStorage};
use crate::text_to_speech::{TextToSpeech, TextToSpeechStorage};
use crate::translation::{Translation, TranslationStorage};
use diesel::QueryResult;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum SubcriptionStorageLimit {
    Translation,
    Checkbot,
    TextToSpeech,
    SpeechToText,
}

impl fmt::Display for SubcriptionStorageLimit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub enum SubcriptionLimit {
    History,
    Storage,
    Translation,
    Checkbot,
    TextToSpeech,
    SpeechToText,
}

impl fmt::Display for SubcriptionLimit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SubcriptionLimit {
    pub fn find_user_subscription_limit_count(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        limit_type: &Self,
    ) -> Option<i64> {
        let limit: Option<i32> =
            match LanguageaiSubscription::find_user_active_subscription(pool, user_id) {
                Ok(subscription) => match limit_type {
                    Self::History => subscription.history_limit,
                    Self::Storage => subscription.storage_limit,
                    Self::Translation => subscription.translation_limit,
                    Self::Checkbot => subscription.checkbot_limit,
                    Self::SpeechToText => subscription.speech_to_text_limit,
                    Self::TextToSpeech => subscription.text_to_speech_limit,
                },
                Err(_) => {
                    match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(pool, &1) {
                        Ok(free_plan) => match limit_type {
                            Self::History => free_plan.history_limit,
                            Self::Storage => free_plan.storage_limit,
                            Self::Translation => free_plan.translation_limit,
                            Self::Checkbot => free_plan.checkbot_limit,
                            Self::SpeechToText => free_plan.speech_to_text_limit,
                            Self::TextToSpeech => free_plan.text_to_speech_limit,
                        },
                        Err(_) => None,
                    }
                }
            };

        limit.map(|value| value as i64)
    }

    pub(super) fn find_user_subscription_usage_count(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        limit_type: &Self,
        storage_limit_type: &Option<SubcriptionStorageLimit>,
    ) -> QueryResult<i64> {
        match storage_limit_type {
            Some(storage_type) => match storage_type {
                SubcriptionStorageLimit::Translation => {
                    TranslationStorage::count_user_translation_storage(pool, user_id)
                }
                SubcriptionStorageLimit::Checkbot => {
                    CheckbotStorage::count_checkbot_storage(pool, user_id)
                }
                SubcriptionStorageLimit::TextToSpeech => {
                    TextToSpeechStorage::count_tts_storage(pool, user_id)
                }
                SubcriptionStorageLimit::SpeechToText => {
                    SpeechToTextStorage::count_storage(pool, user_id)
                }
            },
            None => match limit_type {
                Self::Translation => Translation::count_current_month_translation(pool, user_id),
                Self::Checkbot => Checkbot::count_current_month_checkbot(pool, user_id),
                Self::TextToSpeech => {
                    TextToSpeech::count_current_month_text_to_speech(pool, user_id)
                }
                Self::SpeechToText => {
                    match SpeechToText::count_current_month_speech_to_text_minutes(pool, user_id) {
                        Ok(minutes_option) => match minutes_option {
                            Some(minutes) => Ok(minutes),
                            None => Ok(0),
                        },
                        Err(_) => Ok(0),
                    }
                }
                _ => Ok(0),
            },
        }
    }

    pub fn check_user_exceed_limit(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        limit_type: &Self,
        storage_limit_type: &Option<SubcriptionStorageLimit>,
    ) -> bool {
        match Self::find_user_subscription_limit_count(pool, user_id, limit_type) {
            Some(limit_count) => {
                match Self::find_user_subscription_usage_count(
                    pool,
                    user_id,
                    limit_type,
                    storage_limit_type,
                ) {
                    Ok(usage_count) => usage_count >= limit_count,
                    Err(_) => false,
                }
            }
            None => false,
        }
    }
}
