use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Event {}

/// not complete but all i care about lol
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: usize,
    pub name: Option<String>,
}

/// not complete but all i care about lol
#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub id: usize,
    pub name: Option<String>,
    pub description: Option<String>,
    pub html_url: Option<String>,
    pub due_at: Option<DateTime<Utc>>,
}

pub async fn get_courses(canvas_url: &str) -> Result<Vec<Course>> {
    Ok(reqwest::Client::new()
        .get(&format!("{}/api/v1/courses", canvas_url))
        .bearer_auth(std::env::var("CANVAS_TOKEN")?)
        .send()
        .await?
        .json()
        .await?)
}

pub async fn get_assignments(canvas_url: &str, course_id: usize) -> Result<Vec<Assignment>> {
    Ok(reqwest::Client::new()
        .get(&format!(
            "{}/api/v1/courses/{}/assignments",
            canvas_url, course_id
        ))
        .bearer_auth(std::env::var("CANVAS_TOKEN")?)
        .send()
        .await?
        .json()
        .await?)
}
