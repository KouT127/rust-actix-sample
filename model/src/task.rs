use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub is_done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
