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
    #[arg(short = 'l', long, value_parser = clap::value_parser!(Latitude), required_unless_present = "input_file")]
    latitude: Option<Latitude>,

    #[arg(short = 't', long, value_parser = clap::value_parser!(Longitude), required_unless_present = "input_file")]
    longitude: Option<Longitude>,

    #[arg(short, long, value_parser = clap::value_parser!(Elevation), default_value = "0")]
    elevation: Elevation,

    #[arg(short, long, value_parser = clap::value_parser!(Gmt), required_unless_present = "input_file")]
    gmt: Option<Gmt>,

    #[arg(short, long, value_enum, default_value_t = Method::Isna)]
    method: Method,

    #[arg(short, long, value_parser = clap::value_parser!(NaiveDate))]
    start_date: Option<NaiveDate>,

    #[arg(short = 'n', long, value_parser = clap::value_parser!(NaiveDate))]
    end_date: Option<NaiveDate>,

    #[arg(short, long)]
    input_file: Option<String>,

    #[arg(short, long)]
    output_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonParams {
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
        let file_data = fs::read_to_string(input_file).expect("Could not read the input file!");
        let json_params: JsonParams = serde_json::from_str(&file_data)
            .expect("Could not deserialize JSON data from the input file!");

        params = json_params.params;
        location = json_params.location;
        date_range = json_params.date_range;
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

        let file = File::create("JsonParams.json").expect("Could not create JsonParams.json!");
        serde_json::to_writer(
            file,
            &JsonParams {
                params: params.clone(),
                location,
                date_range: date_range.clone(),
            },
        )
        .expect("Could not serialize JSON data to JsonParams.json!");
    }

    let pts_by_date = prayer_times_dt_rng(&params, location, &date_range);

    if let Some(output_file) = cli_args.output_file {
        let file = File::create(output_file).expect("Could not create the output file!");
        serde_json::to_writer(file, &pts_by_date)
            .expect("Could not serialize JSON data to the output file!");
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
