use std::fs::{self, File};

use chrono::{Local, NaiveDate};
use clap::Parser;
use islamic_prayer_times::{
    prayer_times_dt_rng, Coordinates, DateRange, Elevation, Gmt, HijriDate, Latitude, Location,
    Longitude, Method, Params,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Latitude of the geographical location (required unless the --input_file or -i cli parameter is specified)
    #[arg(short = 'l', long, value_parser = clap::value_parser!(Latitude), required_unless_present = "input_file")]
    latitude: Option<Latitude>,

    /// Longitude of the geographical location (required unless the --input_file or -i cli parameter is specified)
    #[arg(short = 't', long, value_parser = clap::value_parser!(Longitude), required_unless_present = "input_file")]
    longitude: Option<Longitude>,

    /// Elevation of the geographical location (default value = 0m)
    #[arg(short, long, value_parser = clap::value_parser!(Elevation), default_value = "0")]
    elevation: Elevation,

    /// Greenwich Mean Time (GMT) of the geographical location (required unless the --input_file or -i cli parameter is specified)
    #[arg(short, long, value_parser = clap::value_parser!(Gmt), required_unless_present = "input_file")]
    gmt: Option<Gmt>,

    /// Calculation method
    #[arg(short, long, value_enum, default_value_t = Method::Isna)]
    method: Method,

    /// Start date
    #[arg(short, long, value_parser = clap::value_parser!(NaiveDate))]
    start_date: Option<NaiveDate>,

    /// End date
    #[arg(short = 'n', long, value_parser = clap::value_parser!(NaiveDate))]
    end_date: Option<NaiveDate>,

    /// The path to the JSON parameters file
    #[arg(short, long)]
    input_file: Option<String>,

    /// The path to the JSON output file
    #[arg(short, long)]
    output_file: Option<String>,

    /// The path to the JSON output file that captures the cli parameters when the --input_file or -i cli parameter is not specified.
    #[arg(short, long)]
    params_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParamsConfig {
    params: Params,
    location: Location,
    date_range: DateRange,
}

fn main() {
    let cli_args = CliArgs::parse();

    let params: Params;
    let location: Location;
    let date_range: DateRange;

    if let Some(input_file) = cli_args.input_file {
        let file_data = fs::read_to_string(&input_file)
            .expect(&format!("Could not read input file {}", &input_file));
        let params_config: ParamsConfig = serde_json::from_str(&file_data).expect(&format!(
            "Could not deserialize JSON data from input file {}",
            &input_file
        ));

        params = params_config.params;
        location = params_config.location;
        date_range = params_config.date_range;
    } else {
        params = Params::new(cli_args.method);
        let coords = Coordinates::new(
            cli_args.latitude.unwrap(),
            cli_args.longitude.unwrap(),
            cli_args.elevation,
        );
        location = Location {
            coords,
            gmt: cli_args.gmt.unwrap(),
        };

        // If neither start date nor end date is specified, then determine prayer times for today.
        // If only one of start date or end date is specified, then determine prayer times for the date
        // range where today is the other bound.
        // If both start date and end date is specified, then determine prayer times for the date range.
        let today = Local::now().date_naive();
        let start_date = if let Some(date) = cli_args.start_date {
            date
        } else {
            today
        };
        let end_date = if let Some(date) = cli_args.end_date {
            date
        } else {
            start_date
        };

        date_range = DateRange::from(start_date..=end_date);

        if let Some(params_file) = cli_args.params_file {
            let file = File::create(&params_file).expect(&format!(
                "Could not create params config file {}",
                &params_file
            ));
            serde_json::to_writer(
                file,
                &ParamsConfig {
                    params: params.clone(),
                    location,
                    date_range: date_range.clone(),
                },
            )
            .expect(&format!(
                "Could not serialize JSON data to params config file {}",
                &params_file,
            ));
        }
    }

    let pts_by_date = prayer_times_dt_rng(&params, location, &date_range);

    if let Some(output_file) = cli_args.output_file {
        let file = File::create(&output_file)
            .expect(&format!("Could not create output file {}", &output_file));
        serde_json::to_writer(file, &pts_by_date).expect(&format!(
            "Could not serialize JSON data to output file {}",
            &output_file
        ));
    } else {
        for pts_for_date in pts_by_date {
            let hijri_date = HijriDate::from(pts_for_date.0);
            println!(
                "\n{} ({})",
                hijri_date,
                pts_for_date.0.format("%A, %B %d, %Y")
            );
            for pts in pts_for_date.1 {
                if pts.1.is_ok() {
                    println!("  {}: {}", pts.0, pts.1.unwrap());
                } else {
                    println!("  {}: Invalid", pts.0);
                }
            }
        }
    }
}
