mod canvas;

use crate::canvas::get_assignments;
use color_eyre::eyre::Result;
use ics::components::Property;
use ics::{Event, ICalendar};
use sha2::{Digest, Sha256, Sha512};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
struct Args {
    #[structopt(short, long, default_value = "https://canvas.instructure.com")]
    canvas_url: String,
    #[structopt(short, long, default_value = "assignments.ics")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Args::from_args();
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let courses = canvas::get_courses(&opts.canvas_url).await?;
    let assignments = futures::future::join_all(
        courses
            .iter()
            .map(|x| get_assignments(&opts.canvas_url, x.id)),
    )
    .await
    .into_iter()
    .filter_map(|x| x.ok())
    .collect::<Vec<_>>();

    let cal = assignments
        .into_iter()
        .flat_map(|x| x.into_iter())
        .filter_map(|y| {
            let name = y.name.unwrap_or("".to_string());

            let hash = format!(
                "{:x}",
                Sha256::new()
                    .chain(&name.bytes().collect::<Vec<_>>())
                    .chain(
                        &y.due_at?
                            .format("%Y%m%dT%H%M%SZ")
                            .to_string()
                            .bytes()
                            .collect::<Vec<_>>(),
                    )
                    .finalize()
            );

            let mut ev = Event::new(hash, y.due_at?.format("%Y%m%dT%H%M%SZ").to_string());
            ev.push(Property::new("SUMMARY", name));
            ev.push(Property::new(
                "DTSTART",
                y.due_at?.format("%Y%m%dT%H%M%SZ").to_string(),
            ));
            ev.push(Property::new(
                "DTEND",
                y.due_at?.format("%Y%m%dT%H%M%SZ").to_string(),
            ));
            if let Some(url) = y.html_url {
                ev.push(Property::new("LOCATION", url));
            }
            Some(ev)
        })
        .fold(ICalendar::new("2.0", "ics-rs"), |mut acc, ev| {
            acc.add_event(ev);
            acc
        });

    cal.save_file(&opts.output)?;
    Ok(())
}
