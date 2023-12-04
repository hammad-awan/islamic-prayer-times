use std::fs::{self, File};

use chrono::Local;
use clap::Parser;
use cli::{CliArgs, ParamsConfig};
use islamic_prayer_times::{
    prayer_times_dt_rng_block, Coordinates, DateRange, HijriDate, Location, Params,
};

mod cli;

fn main() {
    let cli_args = CliArgs::parse();

    let params: Params;
    let location: Location;
    let date_range: DateRange;

    if let Some(input_file) = cli_args.input_file {
        // Read calculation parameters from JSON input file.
        let file_data = fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Could not read input file {}", &input_file));
        let params_config: ParamsConfig = serde_json::from_str(&file_data).unwrap_or_else(|_| {
            panic!(
                "Could not deserialize JSON data from input file {}",
                &input_file
            )
        });

        params = params_config.params;
        location = params_config.location;
        date_range = params_config.date_range;
    } else {
        // Read calculation parameters from command line arguments.
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
            // Write calculation parameters as JSON file.
            let file = File::create(&params_file)
                .unwrap_or_else(|_| panic!("Could not create params config file {}", &params_file));
            serde_json::to_writer(
                file,
                &ParamsConfig {
                    params: params.clone(),
                    location,
                    date_range: date_range.clone(),
                },
            )
            .unwrap_or_else(|_| {
                panic!(
                    "Could not serialize JSON data to params config file {}",
                    &params_file,
                )
            });
        }
    }

    let pts_by_date = prayer_times_dt_rng_block(&params, location, &date_range, 40);

    if let Some(output_file) = cli_args.output_file {
        // Write prayer times as JSON file.
        let file = File::create(&output_file)
            .unwrap_or_else(|_| panic!("Could not create output file {}", &output_file));
        serde_json::to_writer(file, &pts_by_date).unwrap_or_else(|_| {
            panic!(
                "Could not serialize JSON data to output file {}",
                &output_file
            )
        });
    } else {
        // Display prayer times in terminal.
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
