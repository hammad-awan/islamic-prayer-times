use std::{
    collections::BTreeMap,
    fs::{self, File},
};

use chrono::{Local, NaiveDate};
use clap::Parser;
use cli::{CliArgs, ParamsConfig};
use islamic_prayer_times::{
    prayer_times_dt_rng_block, Coordinates, DateRange, HijriDate, Location, Params, Prayer,
    PrayerTime,
};

mod cli;

fn main() {
    let cli_args = CliArgs::parse();

    let params_config = if let Some(input_file_path) = cli_args.input_file_path {
        read_params_file(&input_file_path)
    } else {
        let params_config = read_params_cli(&cli_args);

        if let Some(params_file_path) = cli_args.params_file_path {
            write_params_file(&params_config, &params_file_path);
        }

        params_config
    };

    let pts_by_date = prayer_times_dt_rng_block(
        &params_config.params,
        params_config.location,
        &params_config.date_range.unwrap(),
        365,
    );

    if let Some(output_file_path) = cli_args.output_file_path {
        write_prayer_times_file(&pts_by_date, &output_file_path);
    } else {
        write_prayer_times_terminal(&pts_by_date);
    }
}

fn read_params_file(input_file_path: &str) -> ParamsConfig {
    // Read the geographical and calculation parameters as JSON from the input file.

    let file_data = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read the geographical and calculation parameters from the file {}",
            &input_file_path
        )
    });

    let mut params_config: ParamsConfig = serde_json::from_str(&file_data).unwrap_or_else(|_| {
        panic!(
            "Failed to deserialize the geographical and calculation parameters as JSON from the file {}",
            &input_file_path
        )
    });

    if params_config.date_range.is_none() {
        let today = Local::now().date_naive();
        params_config.date_range = Some(DateRange::from(today..=today));
    }

    params_config
}

fn read_params_cli(cli_args: &CliArgs) -> ParamsConfig {
    // Read the geographical and calculation parameters from the command line arguments.

    let params = Params::new(cli_args.method);
    let coords = Coordinates::new(
        cli_args.latitude.unwrap(),
        cli_args.longitude.unwrap(),
        cli_args.elevation,
    );
    let location = Location {
        coords,
        gmt: cli_args.gmt.unwrap(),
    };

    // If neither the start date nor the end date is specified, then determine the prayer times for today.
    // If only either the start date or the end date is specified, then set the unspecified date to today
    // and determine the prayer times for the resulting date range.
    // If both the start date and the end date are specified, then determine the prayer times for the date range.
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

    ParamsConfig {
        params,
        location,
        date_range: Some(DateRange::from(start_date..=end_date)),
    }
}

fn write_params_file(params_config: &ParamsConfig, params_file_path: &str) {
    // Write the calculation parameters as JSON to the parameters file.

    let file = File::create(&params_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to create the geographical and calculation parameters file {}",
            &params_file_path
        )
    });

    serde_json::to_writer(file, &params_config)
    .unwrap_or_else(|_| {
        panic!(
            "Failed to serialize the geographical and calculation parameters as JSON to the file {}",
            &params_file_path,
        )
    });
}

fn write_prayer_times_file(
    pts_by_date: &BTreeMap<NaiveDate, BTreeMap<Prayer, Result<PrayerTime, ()>>>,
    output_file: &str,
) {
    // Write the calculated prayer times as JSON to the output file.

    let file = File::create(&output_file).unwrap_or_else(|_| {
        panic!(
            "Failed to create the calculated prayer times output file {}",
            &output_file
        )
    });

    serde_json::to_writer(file, &pts_by_date).unwrap_or_else(|_| {
        panic!(
            "Failed to serialize the calculated prayer times as JSON to the file {}",
            &output_file
        )
    });
}

fn write_prayer_times_terminal(
    pts_by_date: &BTreeMap<NaiveDate, BTreeMap<Prayer, Result<PrayerTime, ()>>>,
) {
    // Display the calculated prayer times in the terminal.

    for pts_for_date in pts_by_date {
        let hijri_date = HijriDate::from(*pts_for_date.0);
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
