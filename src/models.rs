use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Image {
    pub id: i32,
    pub image_url: String,
}

#[derive(FromRow)]
pub struct GoldStar {
    pub id: i64,
    pub number_of_stars: i32,
    pub given_stars: i32,
    pub received_stars: i32,
    pub last_free_star: Option<NaiveDateTime>,
}

#[derive(FromRow)]
pub struct SupportFAQ {
    pub id: String,
    pub answer: String,
    pub guild_id: i64,
}

#[derive(FromRow)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: Option<String>,
    pub user_id: i64,
    pub message_id: Option<i64>,
}

#[derive(FromRow)]
pub struct ReactionRole {
    pub id: i32,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub role_id: i64,
    pub emoji: String,
}

#[derive(FromRow)]
pub struct Infraction {
    pub id: i32,
    pub user_id: i64,
    pub username: String,
    pub guild_id: i64,
    pub infraction_type: String,
    pub moderator_id: i64,
    pub moderator_username: String,
    pub points: i32,
    pub reason: String,
    pub created_at: NaiveDateTime,
}
