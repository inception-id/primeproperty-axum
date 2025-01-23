use crate::db::DbPool;
use crate::languageai_subscriptions::plans::LanguageaiSubscriptionPlan;
use crate::languageai_subscriptions::LanguageaiSubscription;
use crate::translation::Translation;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum SubcriptionLimit {
    HistoryLimit,
    StorageLimit,
    TranslationLimit,
    CheckbotLimit,
    TextToSpeechLimit,
    SpeechToTextLimit,
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
        let mut limit: Option<i32> = None;
        if let Ok(subscription) =
            LanguageaiSubscription::find_user_active_subscription(pool, user_id)
        {
            limit = match limit_type {
                Self::HistoryLimit => subscription.history_limit,
                Self::StorageLimit => subscription.storage_limit,
                Self::TranslationLimit => subscription.translation_limit,
                Self::CheckbotLimit => subscription.checkbot_limit,
                Self::SpeechToTextLimit => subscription.speech_to_text_limit,
                Self::TextToSpeechLimit => subscription.text_to_speech_limit,
            }
        } else {
            limit = match LanguageaiSubscriptionPlan::find_subscription_plan_by_id(pool, &1) {
                Ok(free_plan) => match limit_type {
                    Self::HistoryLimit => free_plan.history_limit,
                    Self::StorageLimit => free_plan.storage_limit,
                    Self::TranslationLimit => free_plan.translation_limit,
                    Self::CheckbotLimit => free_plan.checkbot_limit,
                    Self::SpeechToTextLimit => free_plan.speech_to_text_limit,
                    Self::TextToSpeechLimit => free_plan.text_to_speech_limit,
                },
                Err(_) => None,
            }
        }

        limit.map(|value| value as i64)
    }

    pub(super) fn check_user_exceed_limit(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        limit_type: &Self,
    ) -> bool {
        match Self::find_user_subscription_limit_count(pool, user_id, limit_type) {
            Some(limit_count) => match limit_type {
                SubcriptionLimit::TranslationLimit => {
                    match Translation::count_current_month_translation(&pool, &user_id) {
                        Ok(value) => value >= limit_count,
                        Err(_) => false,
                    }
                }
                _ => false,
            },
            None => false,
        }
    }
}
