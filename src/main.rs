use chrono::{Local, NaiveDate};
use clap::Parser;
use islamic_prayer_times::{
    prayer_times_dt_rng, Coordinates, DateRange, Elevation, Gmt, HijriDate, Latitude, Location,
    Longitude, Method, Params, Prayer,
};
use strum::IntoEnumIterator;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short = 'l', long, value_parser=clap::value_parser!(Latitude))]
    latitude: Latitude,
    #[arg(short = 'o', long, value_parser=clap::value_parser!(Longitude))]
    longitude: Longitude,
    #[arg(short, long, value_parser=clap::value_parser!(Elevation), default_value = "0")]
    elevation: Elevation,
    #[arg(short, long, value_parser=clap::value_parser!(Gmt))]
    gmt: Gmt,
    #[arg(short, long, value_enum, default_value_t = Method::Isna)]
    method: Method,
    #[arg(short, long, value_parser=clap::value_parser!(NaiveDate))]
    start_date: Option<NaiveDate>,
    #[arg(short = 'n', long, value_parser=clap::value_parser!(NaiveDate))]
    end_date: Option<NaiveDate>,
}

fn main() {
    let cli_args = CliArgs::parse();

    let params = Params::new(cli_args.method);
    let coords = Coordinates::new(cli_args.latitude, cli_args.longitude, cli_args.elevation);
    let location = Location {
        coords,
        gmt: cli_args.gmt,
    };

    // If neither start date nor end date is specified, determine prayer times for today.
    // If only one of start date or end date is specified, determine prayer times for the date
    // range where today is the other bound.
    // If both start date and end date is specified, determine prayer times for the date range.
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

    let date_range = DateRange::new(start_date, end_date);
    let pts_by_date = prayer_times_dt_rng(&params, location, date_range);

    for pt_by_date in pts_by_date {
        let hijri_date = HijriDate::from(pt_by_date.0);
        println!(
            "\n{} ({})",
            hijri_date,
            pt_by_date.0.format("%A, %B %d, %Y")
        );
        for prayer in Prayer::iter() {
            let pt_res = pt_by_date.1.get(&prayer).unwrap();
            if pt_res.is_ok() {
                println!("  {}: {}", prayer, pt_res.unwrap());
            } else {
                println!("  {}: Invalid", prayer);
            }
        }
    }
}
