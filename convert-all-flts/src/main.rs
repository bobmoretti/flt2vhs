use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::*;
use chrono::prelude::*;
use log::*;
use structopt::StructOpt;

/// Converts all FLT files in the directory to VHS,
/// then waits for BMS to make more.
#[derive(Debug, StructOpt)]
#[structopt(verbatim_doc_comment)]
struct Args {
    /// Verbosity (-v, -vv, -vvv, etc.). Defaults to `-v`
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long, case_insensitive = true, default_value = "auto")]
    #[structopt(name = "always/auto/never")]
    color: logsetup::Color,

    /// Prepend ISO-8601 timestamps to all messages
    /// (from --verbose). Useful for benchmarking.
    #[structopt(short, long, verbatim_doc_comment)]
    timestamps: bool,

    /// Switch to this working directory (e.g., BMS/User/Acmi)
    /// before doing anything else.
    #[structopt(short = "C", long, name = "path")]
    #[structopt(verbatim_doc_comment)]
    directory: Option<PathBuf>,

    /// The program to convert FLT files to VHS
    /// Assumed usage is `<converter> input.flt`
    #[structopt(long, default_value = "flt2vhs.exe", name = "program")]
    #[structopt(verbatim_doc_comment)]
    converter: PathBuf,

    /// Don't convert FLT files to VHS once they've been moved.
    #[structopt(short, long)]
    no_convert: bool,

    /// Keep FLT files instead of deleting them after converting them.
    /// Ignored if --no-convert is given
    #[structopt(short, long, verbatim_doc_comment)]
    keep: bool,
}

fn main() {
    run().unwrap_or_else(|e| {
        error!("{:?}", e);
        std::process::exit(1);
    });
}

fn run() -> Result<()> {
    let args = Args::from_args();
    logsetup::init_logger(std::cmp::max(1, args.verbose), args.timestamps, args.color);

    if let Some(change_to) = &args.directory {
        env::set_current_dir(change_to).with_context(|| {
            format!("Couldn't set working directory to {}", change_to.display())
        })?;
    }

    rename_and_convert(&args)
}

fn rename_and_convert(args: &Args) -> Result<()> {
    let mut to_rename = Vec::new();

    let cwd = env::current_dir()?;
    for f in fs::read_dir(cwd)? {
        let entry = f?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("flt")) && entry.metadata()?.is_file() {
            trace!("Found .flt file {}", path.display());
            to_rename.push(path);
        }
    }

    let renamed_flights = to_rename
        .iter()
        .map(|f| rename_flt(f))
        .collect::<Result<Vec<_>>>()?;

    convert_flts(args, &renamed_flights)?;
    Ok(())
}

fn rename_flt(to_rename: &Path) -> Result<PathBuf> {
    let rename_to = timestamp_name(&to_rename);
    debug!("Trying to rename {}...", to_rename.display());
    fs::rename(&to_rename, &rename_to)
        .with_context(|| format!("Renaming {} failed", to_rename.display()))?;
    info!("Renamed {} to {}", to_rename.display(), rename_to.display());
    Ok(rename_to)
}

const MOVED_SUFFIX: &str = ".moved";

// _TOCTOU: The Function_, but let's assume nothing's making a bunch of FLT files
// in the exact same second.
fn timestamp_name(to_rename: &Path) -> PathBuf {
    use std::os::windows::fs::MetadataExt;

    let now = Local::now();

    match fs::metadata(to_rename).map(|meta| windows_timestamp(meta.creation_time())) {
        Ok(Some(ct)) => {
            let local = ct.with_timezone(now.offset());
            PathBuf::from(format!(
                "{}.flt{}",
                local.format("%Y-%m-%d_%H-%M-%S"),
                MOVED_SUFFIX
            ))
        }
        Ok(None) | Err(_) => {
            trace!(
                "Couldn't get creation time of {}. Falling back to current time",
                to_rename.display()
            );
            PathBuf::from(format!(
                "{}.flt{}",
                now.format("%Y-%m-%d_%H-%M-%S"),
                MOVED_SUFFIX
            ))
        }
    }
}

fn windows_timestamp(ts: u64) -> Option<DateTime<Utc>> {
    // Windows returns 100ns intervals since January 1, 1601
    const TICKS_PER_SECOND: u64 = 1_000_000_000 / 100;

    if ts == 0 {
        None
    } else {
        let seconds = ts / TICKS_PER_SECOND;
        let nanos = (ts % TICKS_PER_SECOND) * 100;

        Some(
            Utc.ymd(1601, 1, 1).and_hms(0, 0, 0)
                + chrono::Duration::seconds(seconds as i64)
                + chrono::Duration::nanoseconds(nanos as i64),
        )
    }
}

fn path_list(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| p.to_string_lossy())
        .collect::<Vec<_>>()
        .join(", ")
}

fn convert_flts(args: &Args, flts: &[PathBuf]) -> Result<()> {
    debug!(
        "Converting {:?} with {}",
        path_list(flts),
        args.converter.display()
    );

    let mut proc = std::process::Command::new(&args.converter);
    // Add a verbosity flag if it's flt2vhs.
    // Don't for other programs since we shouldn't assume how their flags work
    if args.converter == Path::new("flt2vhs.exe") {
        proc.arg("-v");
        if !args.keep {
            proc.arg("--delete");
        }
    }
    proc.args(flts);
    let exit_status = proc
        .status()
        .with_context(|| format!("Couldn't run {}", args.converter.display()))?;
    if exit_status.success() {
        Ok(())
    } else {
        bail!(
            "{} failed to convert {}",
            args.converter.display(),
            path_list(flts)
        );
    }
}
