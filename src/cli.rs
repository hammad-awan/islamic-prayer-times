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
    /// Required Greenwich Mean Time of geographical location unless --input_file or -i command line parameter is specified
    #[arg(short, long, value_parser = clap::value_parser!(Gmt), required_unless_present = "input_file")]
    pub gmt: Option<Gmt>,

    /// Required latitude of geographical location unless --input_file or -i command line parameter is specified
    #[arg(short = 'l', long, value_parser = clap::value_parser!(Latitude), required_unless_present = "input_file")]
    pub latitude: Option<Latitude>,

    /// Required longitude of geographical location unless --input_file or -i command line parameter is specified
    #[arg(short = 't', long, value_parser = clap::value_parser!(Longitude), required_unless_present = "input_file")]
    pub longitude: Option<Longitude>,

    /// Optional elevation of geographical location the default value of which is 0m
    #[arg(short, long, value_parser = clap::value_parser!(Elevation), default_value = "0")]
    pub elevation: Elevation,

    /// Optional calculation method the default value of which is Method::Isna
    #[arg(short, long, value_enum, default_value_t = Method::Isna)]
    pub method: Method,

    /// Optional start date the default value of which is today
    #[arg(short, long, value_parser = clap::value_parser!(NaiveDate))]
    pub start_date: Option<NaiveDate>,

    /// Optional end date the default value of which is today
    #[arg(short = 'n', long, value_parser = clap::value_parser!(NaiveDate))]
    pub end_date: Option<NaiveDate>,

    /// Optional path to JSON file from which to read geographical location and calculation method parameters
    #[arg(short, long)]
    pub input_file: Option<String>,

    /// Optional path to file to write prayer times as JSON
    #[arg(short, long)]
    pub output_file: Option<String>,

    /// Optional path to file to write command line geographical location and calculation parameters as JSON
    /// when --input_file or -i command line parameter is not specified
    #[arg(short, long)]
    pub params_file: Option<String>,
}
