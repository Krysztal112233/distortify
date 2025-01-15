use std::{
    env,
    io::{Cursor, Write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use colored::Colorize;
use image::ImageReader;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use iter_tools::Itertools;
use log::{error, info};
use rand::{thread_rng, Rng};
use tokio::sync::Semaphore;
use util::{img::ImageHelper, path::PathHelper, ImageAction};
use waitgroup::WaitGroup;
use walkdir::WalkDir;

mod error;
mod util;

#[tokio::main(flavor = "multi_thread", worker_threads = 64)]
async fn main() -> anyhow::Result<()> {
    if let Err(env::VarError::NotPresent) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    let logger = pretty_env_logger::formatted_builder()
        .parse_default_env()
        .build();
    let pb = {
        let multi = MultiProgress::new();
        LogWrapper::new(multi.clone(), logger).try_init().unwrap();
        Arc::new(multi.add(ProgressBar::no_length()))
    };

    pb.set_style(
        ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({percent}%) {msg}")?.progress_chars("#>-"),
    );

    let dirs = WalkDir::new(".")
        .follow_root_links(true)
        .max_depth(1)
        .into_iter()
        .flatten()
        .filter(|it| it.file_type().is_dir() || it.file_type().is_symlink())
        .map(|it| it.into_path())
        .map(|it| it.display().to_string())
        .collect_vec();

    let path = inquire::Select::new("Where the input sources", dirs).prompt()?;

    let chance = inquire::Text::new("The ratio wanna be processed")
        .prompt()?
        .parse::<f32>()?;

    let targets = || {
        WalkDir::new(path.clone())
            .follow_root_links(true)
            .into_iter()
            .flatten()
            .filter(|it| it.file_type().is_file())
            .map(|it| it.into_path())
    };

    pb.set_length(targets().count() as u64);

    let semaphore = Arc::new(Semaphore::new(64));
    let wg = WaitGroup::new();

    let targets = targets();
    for img in targets {
        let w = wg.worker();
        let pb = pb.clone();
        let semaphore = semaphore.clone();

        tokio::spawn(async move {
            if let Err(err) = semaphore.acquire().await {
                error!("While acquire semaphore: {}", err)
            };

            if let Err(err) = task(pb, img.clone(), chance).await {
                error!(
                    "While processing file {}: {}",
                    img.display().to_string().bold().underline(),
                    err
                )
            };
            drop(w);
        });
    }

    wg.wait().await;

    Ok(())
}

async fn task(pb: Arc<ProgressBar>, path: PathBuf, ratio: f32) -> Result<()> {
    if thread_rng().gen_range(0.0..1.0) >= ratio {
        pb.inc(1);

        return Ok(());
    }

    let (act, itered) = ImageAction::random();

    for itered in 0..itered {
        let write_to = path.modified(&act, itered);

        info!(
            "Processing file {}, write to {}",
            path.display().to_string().bold().underline(),
            write_to.display().to_string().underline().bold()
        );

        let img = ImageReader::open(&path)?;
        let format = img.format().unwrap();
        let img = img.decode()?.to_rgb8().proc(&act);

        let mut buffer = Cursor::new(Vec::new());
        img.write_to(&mut buffer, format)?;

        let mut file = std::fs::File::create(write_to)?;
        file.write_all(buffer.get_ref())?;
        file.flush()?;
        drop(file);
    }

    pb.inc(1);

    Ok(())
}
