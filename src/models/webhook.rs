use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::models::User;
use crate::schema::webhooks;

use url::Url;

#[derive(Debug, Queryable, AsChangeset, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Webhook {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "webhooks"]
pub struct NewWebhook {
    pub user_id: i32,
    pub url: String,
}

impl NewWebhook {
    pub fn new(user_id: i32, url: &Url) -> Self {
        // TODO(mdtro): check to make sure the webhook is HTTP/HTTPS
        NewWebhook {
            user_id,
            url: url.to_string(),
        }
    }
}
