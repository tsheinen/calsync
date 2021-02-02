mod canvas;

use crate::canvas::{get_assignments, Assignment};
use color_eyre::eyre::Result;
use ics::ICalendar;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
struct Args {
    #[structopt(short, long, default_value = "https://canvas.instructure.com")]
    canvas_url: String,
    #[structopt(short, long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Args::from_args();
    dotenv::dotenv();
    pretty_env_logger::init();
    let courses = canvas::get_courses(&opts.canvas_url).await?;
    let cal = futures::future::join_all(
        courses
            .iter()
            .map(|x| get_assignments(&opts.canvas_url, x.id)),
    )
    .await
    .into_iter()
    .filter_map(Result::ok)
    .into_iter()
    .flat_map(Vec::into_iter)
    .filter_map(Assignment::into)
    .fold(ICalendar::new("2.0", "ics-rs"), |mut acc, ev| {
        acc.add_event(ev);
        acc
    });

    if let Some(filename) = opts.output {
        cal.save_file(filename)?;
    } else {
        cal.write(std::io::stdout())?;
    }
    Ok(())
}
