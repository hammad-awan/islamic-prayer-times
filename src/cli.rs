use chrono::NaiveDate;
use clap::Parser;
use islamic_prayer_times::{
    DateRange, Elevation, Gmt, Latitude, Location, Longitude, Method, Params,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsConfig {
    pub params: Params,
    pub location: Location,
    pub date_range: DateRange,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // Required Greenwich Mean Time of geographical location unless --input_file or -i command line parameter is specified.
    #[arg(short, long, value_parser = clap::value_parser!(Gmt), required_unless_present = "input_file_path")]
    pub gmt: Option<Gmt>,

    // Required latitude of geographical location unless --input_file or -i command line parameter is specified.
    #[arg(short = 'l', long, value_parser = clap::value_parser!(Latitude), required_unless_present = "input_file_path")]
    pub latitude: Option<Latitude>,

    // Required longitude of geographical location unless --input_file or -i command line parameter is specified.
    #[arg(short = 't', long, value_parser = clap::value_parser!(Longitude), required_unless_present = "input_file_path")]
    pub longitude: Option<Longitude>,

    // Optional elevation of geographical location the default value of which is 0m.
    #[arg(short, long, value_parser = clap::value_parser!(Elevation), default_value = "0")]
    pub elevation: Elevation,

    // Optional calculation method the default value of which is Method::Isna.
    #[arg(short, long, value_enum, default_value_t = Method::Isna)]
    pub method: Method,

    // Optional start date the default value of which is today.
    #[arg(short, long, value_parser = clap::value_parser!(NaiveDate))]
    pub start_date: Option<NaiveDate>,

    // Optional end date the default value of which is today.
    #[arg(short = 'n', long, value_parser = clap::value_parser!(NaiveDate))]
    pub end_date: Option<NaiveDate>,

    // Optional path to the file to read the JSON geographical location and calculation method parameters.
    #[arg(short, long)]
    pub input_file_path: Option<String>,

    // Optional path to the file to write the calculated prayer times to as JSON.
    #[arg(short, long)]
    pub output_file_path: Option<String>,

    // Optional path to the file to write the geographical location and calculation parameters to as JSON
    // when the --input_file or -i command line parameter is not specified.
    #[arg(short, long)]
    pub params_file_path: Option<String>,
}
