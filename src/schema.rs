// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_status"))]
    pub struct PaymentStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "subscription_period"))]
    pub struct SubscriptionPeriod;
}

diesel::table! {
    ai_system_prompts (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        product_name -> Varchar,
        prompt -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    checkbot (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        instruction -> Varchar,
        ai_system_prompt -> Varchar,
        content -> Text,
        completion -> Text,
    }
}

diesel::table! {
    checkbot_storage (id) {
        id -> Int4,
        user_id -> Uuid,
        checkbot_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        instruction -> Varchar,
        content -> Text,
        updated_completion -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SubscriptionPeriod;
    use super::sql_types::PaymentStatus;

    languageai_subscription_payments (id) {
        id -> Int4,
        user_id -> Uuid,
        languageai_subscription_plan_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        expired_at -> Timestamp,
        amount -> Numeric,
        period -> SubscriptionPeriod,
        status -> PaymentStatus,
        doku_response -> Nullable<Jsonb>,
    }
}

diesel::table! {
    languageai_subscription_plans (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Varchar,
        initial_price -> Numeric,
        discounted_price -> Nullable<Numeric>,
        history_limit -> Nullable<Int4>,
        storage_limit -> Nullable<Int4>,
        translation_limit -> Nullable<Int4>,
        checkbot_limit -> Nullable<Int4>,
        text_to_speech_limit -> Nullable<Int4>,
        speech_to_text_limit -> Nullable<Int4>,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        #[max_length = 2]
        iso_639_1 -> Varchar,
    }
}

diesel::table! {
    speech_to_text (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        audio_url -> Varchar,
        transcription_text -> Text,
        language -> Nullable<Varchar>,
    }
}

diesel::table! {
    speech_to_text_storage (id) {
        id -> Int4,
        user_id -> Uuid,
        speech_to_text_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        audio_url -> Varchar,
        updated_transcription_text -> Text,
        language -> Nullable<Varchar>,
    }
}

diesel::table! {
    text_to_speech (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        input_content -> Text,
        audio_url -> Varchar,
        voice -> Varchar,
    }
}

diesel::table! {
    text_to_speech_storage (id) {
        id -> Int4,
        user_id -> Uuid,
        text_to_speech_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        input_content -> Text,
        audio_url -> Varchar,
        voice -> Varchar,
    }
}

diesel::table! {
    translation (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ai_system_prompt -> Varchar,
        content_language -> Nullable<Varchar>,
        target_language -> Varchar,
        content -> Text,
        completion -> Text,
    }
}

diesel::table! {
    translation_storage (id) {
        id -> Int4,
        user_id -> Uuid,
        translation_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        content_language -> Nullable<Varchar>,
        target_language -> Varchar,
        content -> Text,
        updated_completion -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
    }
}

diesel::joinable!(checkbot -> users (user_id));
diesel::joinable!(checkbot_storage -> checkbot (checkbot_id));
diesel::joinable!(checkbot_storage -> users (user_id));
diesel::joinable!(languageai_subscription_payments -> languageai_subscription_plans (languageai_subscription_plan_id));
diesel::joinable!(languageai_subscription_payments -> users (user_id));
diesel::joinable!(speech_to_text -> users (user_id));
diesel::joinable!(speech_to_text_storage -> speech_to_text (speech_to_text_id));
diesel::joinable!(speech_to_text_storage -> users (user_id));
diesel::joinable!(text_to_speech -> users (user_id));
diesel::joinable!(text_to_speech_storage -> text_to_speech (text_to_speech_id));
diesel::joinable!(text_to_speech_storage -> users (user_id));
diesel::joinable!(translation -> users (user_id));
diesel::joinable!(translation_storage -> translation (translation_id));
diesel::joinable!(translation_storage -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ai_system_prompts,
    checkbot,
    checkbot_storage,
    languageai_subscription_payments,
    languageai_subscription_plans,
    languages,
    speech_to_text,
    speech_to_text_storage,
    text_to_speech,
    text_to_speech_storage,
    translation,
    translation_storage,
    users,
);
