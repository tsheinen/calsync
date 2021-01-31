use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use ics::components::Property;
use ics::Event;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

impl Into<Option<Event<'static>>> for Assignment {
    fn into(self) -> Option<Event<'static>> {
        let name = self.name.unwrap_or_else(String::new);

        let hash = format!(
            "{:x}",
            Sha256::new()
                .chain(&name.bytes().collect::<Vec<_>>())
                .chain(
                    &self
                        .due_at?
                        .format("%Y%m%dT%H%M%SZ")
                        .to_string()
                        .bytes()
                        .collect::<Vec<_>>(),
                )
                .finalize()
        );

        Some(
            vec![
                Some(Property::new("SUMMARY", name)),
                Some(Property::new(
                    "DTSTART",
                    self.due_at?.format("%Y%m%dT%H%M%SZ").to_string(),
                )),
                Some(Property::new(
                    "DTEND",
                    self.due_at?.format("%Y%m%dT%H%M%SZ").to_string(),
                )),
                self.html_url.map(|url| Property::new("LOCATION", url)),
            ]
            .into_iter()
            .filter_map(|x| x)
            .fold(
                Event::new(hash, self.due_at?.format("%Y%m%dT%H%M%SZ").to_string()),
                |mut acc, prop| {
                    acc.push(prop);
                    acc
                },
            ),
        )
    }
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
