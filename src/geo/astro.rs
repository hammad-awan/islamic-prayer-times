use crate::angle::LimitAngle;

use super::{coordinates::Coordinates, julian_day::JulianDay};

static TEN_POW_EIGHT: f64 = 10_i32.pow(8) as f64;

// Astronomical Algorithms pg. 418-419
const L0: [(f64, f64, f64); 64] = [
    (175347046., 0., 0.),
    (3341656., 4.6692568, 6283.07585),
    (34894., 4.6261, 12566.1517),
    (3497., 2.7441, 5753.3849),
    (3418., 2.8289, 3.5231),
    (3136., 3.6277, 77713.7715),
    (2676., 4.4181, 7860.4194),
    (2343., 6.1352, 3930.2097),
    (1324., 0.7425, 11506.7698),
    (1273., 2.0371, 529.691),
    (1199., 1.1096, 1577.3435),
    (990., 5.233, 5884.927),
    (902., 2.045, 26.298),
    (857., 3.508, 398.149),
    (780., 1.179, 5223.694),
    (753., 2.533, 5507.553),
    (505., 4.583, 18849.228),
    (492., 4.205, 775.523),
    (357., 2.92, 0.067),
    (317., 5.849, 11790.629),
    (284., 1.899, 796.298),
    (271., 0.315, 10977.079),
    (243., 0.345, 5486.778),
    (206., 4.806, 2544.314),
    (205., 1.869, 5573.143),
    (202., 2.4458, 6069.777),
    (156., 0.833, 213.299),
    (132., 3.411, 2942.463),
    (126., 1.083, 20.775),
    (115., 0.645, 0.98),
    (103., 0.636, 4694.003),
    (102., 0.976, 15720.839),
    (102., 4.267, 7.114),
    (99., 6.21, 2146.17),
    (98., 0.68, 155.42),
    (86., 5.98, 161000.69),
    (85., 1.3, 6275.96),
    (85., 3.67, 71430.7),
    (80., 1.81, 17260.15),
    (79., 3.04, 12036.46),
    (71., 1.76, 5088.63),
    (74., 3.5, 3154.69),
    (74., 4.68, 801.82),
    (70., 0.83, 9437.76),
    (62., 3.98, 8827.39),
    (61., 1.82, 7084.9),
    (57., 2.78, 6286.6),
    (56., 4.39, 14143.5),
    (56., 3.47, 6279.55),
    (52., 0.19, 12139.55),
    (52., 1.33, 1748.02),
    (51., 0.28, 5856.48),
    (49., 0.49, 1194.45),
    (41., 5.37, 8429.24),
    (41., 2.4, 19651.05),
    (39., 6.17, 10447.39),
    (37., 6.04, 10213.29),
    (37., 2.57, 1059.38),
    (36., 1.71, 2352.87),
    (36., 1.78, 6812.77),
    (33., 0.59, 17789.85),
    (30., 0.44, 83996.85),
    (30., 2.74, 1349.87),
    (25., 3.16, 4690.48),
];

// Astronomical Algorithms pg. 419
const L1: [(f64, f64, f64); 34] = [
    (628331966747., 0., 0.),
    (206059., 2.678235, 6283.07585),
    (4303., 2.6351, 12566.1517),
    (425., 1.59, 3.523),
    (119., 5.796, 26.298),
    (109., 2.966, 1577.344),
    (93., 2.59, 18849.23),
    (72., 1.14, 529.69),
    (68., 1.87, 398.15),
    (67., 4.41, 5507.55),
    (59., 2.89, 5223.69),
    (56., 2.17, 155.42),
    (45., 0.4, 796.3),
    (36., 0.47, 775.52),
    (29., 2.65, 7.11),
    (21., 5.34, 0.98),
    (19., 1.85, 5486.78),
    (19., 4.97, 213.3),
    (17., 2.99, 6275.96),
    (16., 0.03, 2544.31),
    (16., 1.43, 2146.17),
    (15., 1.21, 10977.08),
    (12., 2.83, 1748.02),
    (12., 3.26, 5088.63),
    (12., 5.27, 1194.45),
    (12., 2.08, 4694.),
    (11., 0.77, 553.57),
    (10., 1.3, 3286.6),
    (10., 4.24, 1349.87),
    (9., 2.7, 242.73),
    (9., 5.64, 951.72),
    (8., 5.3, 2352.87),
    (6., 2.65, 9437.76),
    (6., 4.67, 4690.48),
];

// Astronomical Algorithms pg. 420
const L2: [(f64, f64, f64); 20] = [
    (52919., 0., 0.),
    (8720., 1.0721, 6283.0758),
    (309., 0.867, 12566.152),
    (27., 0.05, 3.52),
    (16., 5.19, 26.3),
    (16., 3.68, 155.42),
    (10., 0.76, 18849.23),
    (9., 2.06, 77713.77),
    (7., 0.83, 775.52),
    (5., 4.66, 1577.34),
    (4., 1.03, 7.11),
    (4., 3.44, 5573.14),
    (3., 5.14, 796.3),
    (3., 6.05, 5507.55),
    (3., 1.19, 242.73),
    (3., 6.12, 529.69),
    (3., 0.31, 398.15),
    (3., 2.28, 553.57),
    (2., 4.38, 5223.69),
    (2., 3.75, 0.98),
];

// Astronomical Algorithms pg. 420
const L3: [(f64, f64, f64); 7] = [
    (289., 5.844, 6283.076),
    (35., 0., 0.),
    (17., 5.49, 12566.15),
    (3., 5.2, 155.42),
    (1., 4.72, 3.52),
    (1., 5.3, 18849.23),
    (1., 5.97, 242.73),
];

// Astronomical Algorithms pg. 420
const L4: [(f64, f64, f64); 3] = [
    (114.0, 3.142, 0.0),
    (8.0, 4.13, 6283.08),
    (1.0, 3.84, 12566.15),
];

// Astronomical Algorithms pg. 420
const L5: [(f64, f64, f64); 1] = [(1., 3.14, 0.)];

// Astronomical Algorithms pg. 420
const B0: [(f64, f64, f64); 5] = [
    (280., 3.199, 84334.662),
    (102., 5.422, 5507.553),
    (80., 3.88, 5223.69),
    (44., 3.7, 2352.87),
    (32., 4., 1577.34),
];

// Astronomical Algorithms pg. 420
const B1: [(f64, f64, f64); 2] = [(9., 3.9, 5507.55), (6., 1.73, 5223.69)];

// Astronomical Algorithms pg. 420-421
const R0: [(f64, f64, f64); 40] = [
    (100013989., 0., 0.),
    (1670700., 3.0984635, 6283.07585),
    (13956., 3.05525, 12566.1517),
    (3084., 5.1985, 77713.7715),
    (1628., 1.1739, 5753.3849),
    (1576., 2.8469, 7860.4194),
    (925., 5.453, 11506.77),
    (542., 4.564, 3930.21),
    (472., 3.661, 5884.927),
    (346., 0.964, 5507.553),
    (329., 5.9, 5223.694),
    (307., 0.299, 5573.143),
    (243., 4.273, 11790.629),
    (212., 5.847, 1577.344),
    (186., 5.022, 10977.079),
    (175., 3.012, 18849.228),
    (110., 5.055, 5486.778),
    (98., 0.89, 6069.78),
    (86., 5.69, 15720.84),
    (86., 1.27, 161000.69),
    (85., 0.27, 17260.15),
    (63., 0.92, 529.69),
    (57., 2.01, 83996.85),
    (56., 5.24, 71430.7),
    (49., 3.25, 2544.31),
    (47., 2.58, 775.52),
    (45., 5.54, 9437.76),
    (43., 6.01, 6275.96),
    (39., 5.36, 4694.),
    (38., 2.39, 8827.39),
    (37., 0.83, 19651.05),
    (37., 4.9, 12139.55),
    (36., 1.67, 12036.46),
    (35., 1.84, 2942.46),
    (33., 0.24, 7084.9),
    (32., 0.18, 5088.63),
    (32., 1.78, 398.15),
    (28., 1.21, 6286.6),
    (28., 1.9, 6279.55),
    (26., 4.59, 10447.39),
];

// Astronomical Algorithms pg. 421
const R1: [(f64, f64, f64); 10] = [
    (103019., 1.10749, 6283.07585),
    (1721., 1.0644, 12566.1517),
    (702., 3.142, 0.),
    (32., 1.02, 18849.23),
    (31., 2.84, 5507.55),
    (25., 1.32, 5223.69),
    (18., 1.42, 1577.34),
    (10., 5.91, 10977.08),
    (9., 1.42, 6275.96),
    (9., 0.27, 5486.78),
];

// Astronomical Algorithms pg. 421
const R2: [(f64, f64, f64); 6] = [
    (4359., 5.7846, 6283.0758),
    (124., 5.579, 12566.152),
    (12., 3.14, 0.),
    (9., 3.63, 77713.77),
    (6., 1.87, 5573.14),
    (3., 5.47, 18849.),
];

// Astronomical Algorithms pg. 421
const R3: [(f64, f64, f64); 2] = [(145., 4.273, 6283.076), (7., 3.92, 12566.15)];

// Astronomical Algorithms pg. 421
const R4: [(f64, f64, f64); 1] = [(4., 2.56, 6283.08)];

// Astronomical Algorithms pg. 145-146 (Table 22.A)
const PE: [(f64, f64, f64, f64); 63] = [
    (-171996., -174.2, 92025., 8.9),
    (-13187., -1.6, 5736., -3.1),
    (-2274., -0.2, 977., -0.5),
    (2062., 0.2, -895., 0.5),
    (1426., -3.4, 54., -0.1),
    (712., 0.1, -7., 0.),
    (-517., 1.2, 224., -0.6),
    (-386., -0.4, 200., 0.),
    (-301., 0., 129., -0.1),
    (217., -0.5, -95., 0.3),
    (-158., 0., 0., 0.),
    (129., 0.1, -70., 0.),
    (123., 0., -53., 0.),
    (63., 0., 0., 0.),
    (63., 0.1, -33., 0.),
    (-59., 0., 26., 0.),
    (-58., -0.1, 32., 0.),
    (-51., 0., 27., 0.),
    (48., 0., 0., 0.),
    (46., 0., -24., 0.),
    (-38., 0., 16., 0.),
    (-31., 0., 13., 0.),
    (29., 0., 0., 0.),
    (29., 0., -12., 0.),
    (26., 0., 0., 0.),
    (-22., 0., 0., 0.),
    (21., 0., -10., 0.),
    (17., -0.1, 0., 0.),
    (16., 0., -8., 0.),
    (-16., 0.1, 7., 0.),
    (-15., 0., 9., 0.),
    (-13., 0., 7., 0.),
    (-12., 0., 6., 0.),
    (11., 0., 0., 0.),
    (-10., 0., 5., 0.),
    (-8., 0., 3., 0.),
    (7., 0., -3., 0.),
    (-7., 0., 0., 0.),
    (-7., 0., 3., 0.),
    (-7., 0., 3., 0.),
    (6., 0., 0., 0.),
    (6., 0., -3., 0.),
    (6., 0., -3., 0.),
    (-6., 0., 3., 0.),
    (-6., 0., 3., 0.),
    (5., 0., 0., 0.),
    (-5., 0., 3., 0.),
    (-5., 0., 3., 0.),
    (-5., 0., 3., 0.),
    (4., 0., 0., 0.),
    (4., 0., 0., 0.),
    (4., 0., 0., 0.),
    (-4., 0., 0., 0.),
    (-4., 0., 0., 0.),
    (-4., 0., 0., 0.),
    (3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
    (-3., 0., 0., 0.),
];

// Astronomical Algorithms pg. 145-146 (Table 22.A)
const SIN_COEFFICIENT: [(i8, i8, i8, i8, i8); 63] = [
    (0, 0, 0, 0, 1),
    (-2, 0, 0, 2, 2),
    (0, 0, 0, 2, 2),
    (0, 0, 0, 0, 2),
    (0, 1, 0, 0, 0),
    (0, 0, 1, 0, 0),
    (-2, 1, 0, 2, 2),
    (0, 0, 0, 2, 1),
    (0, 0, 1, 2, 2),
    (-2, -1, 0, 2, 2),
    (-2, 0, 1, 0, 0),
    (-2, 0, 0, 2, 1),
    (0, 0, -1, 2, 2),
    (2, 0, 0, 0, 0),
    (0, 0, 1, 0, 1),
    (2, 0, -1, 2, 2),
    (0, 0, -1, 0, 1),
    (0, 0, 1, 2, 1),
    (-2, 0, 2, 0, 0),
    (0, 0, -2, 2, 1),
    (2, 0, 0, 2, 2),
    (0, 0, 2, 2, 2),
    (0, 0, 2, 0, 0),
    (-2, 0, 1, 2, 2),
    (0, 0, 0, 2, 0),
    (-2, 0, 0, 2, 0),
    (0, 0, -1, 2, 1),
    (0, 2, 0, 0, 0),
    (2, 0, -1, 0, 1),
    (-2, 2, 0, 2, 2),
    (0, 1, 0, 0, 1),
    (-2, 0, 1, 0, 1),
    (0, -1, 0, 0, 1),
    (0, 0, 2, -2, 0),
    (2, 0, -1, 2, 1),
    (2, 0, 1, 2, 2),
    (0, 1, 0, 2, 2),
    (-2, 1, 1, 0, 0),
    (0, -1, 0, 2, 2),
    (2, 0, 0, 2, 1),
    (2, 0, 1, 0, 0),
    (-2, 0, 2, 2, 2),
    (-2, 0, 1, 2, 1),
    (2, 0, -2, 0, 1),
    (2, 0, 0, 0, 1),
    (0, -1, 1, 0, 0),
    (-2, -1, 0, 2, 1),
    (-2, 0, 0, 0, 1),
    (0, 0, 2, 2, 1),
    (-2, 0, 2, 0, 1),
    (-2, 1, 0, 2, 1),
    (0, 0, 1, -2, 0),
    (-1, 0, 1, 0, 0),
    (-2, 1, 0, 0, 0),
    (1, 0, 0, 0, 0),
    (0, 0, 1, 2, 0),
    (0, 0, -2, 2, 2),
    (-1, -1, 1, 0, 0),
    (0, 1, 1, 0, 0),
    (0, -1, 1, 2, 2),
    (2, -1, -1, 2, 2),
    (0, 0, 3, 2, 2),
    (2, -1, 0, 2, 2),
];

const EARTH_RADIUS: f64 = 6378140.;

#[derive(Debug, Clone, Copy)]
pub struct Astro {
    pub dra: f64,      // Delta Right Ascension
    pub dec: f64,      // Declination
    pub ra: f64,       // Right Ascension
    pub rsum: f64,     // Sum of periodic values for radius vector R
    pub sid_time: f64, // Sidereal time
}

impl Astro {
    fn new(julian_day: f64) -> Self {
        // Astronomical Algorithms (AA) pg. 143-148
        let j = julian_day - 2451545.;
        // Astronomical Algorithms pg. 143 (22.1)
        let jc = j / 36525.;
        let jm = jc / 10.;
        let mut jms = Astro::pow_series(jm, 5);
        jms.insert(0, 1.);

        let ls = [&L0[..], &L1, &L2, &L3, &L4, &L5];
        let bs = [&B0[..], &B1];
        let rs = [&R0[..], &R1, &R2, &R3, &R4];

        let lsum = Astro::calc_sum(&ls, &jms);
        let bsum = Astro::calc_sum(&bs, &jms);
        let rsum = Astro::calc_sum(&rs, &jms);

        // Astronomical Algorithms pg. 144 (Mean elongation of the Moon from the Sun)
        let jc2 = jc * jc;
        let jc3 = jc2 * jc;
        let d = 297.85036 + 445267.111480 * jc - 0.0019142 * jc2 + jc3 / 189474.;
        // Astronomical Algorithms pg. 144 (Mean anomaly of the Sun)
        let m = 357.52772 + 35999.050340 * jc - 0.0001603 * jc2 - jc3 / 300000.;
        // Astronomical Algorithms pg. 144 (Mean anomaly of the Moon)
        let m_p = 134.96298 + 477198.867398 * jc + 0.0086972 * jc2 + jc3 / 56250.;
        // Astronomical Algorithms pg. 144 (Moon's argument of latitude)
        let f = 93.27191 + 483202.017538 * jc - 0.0036825 * jc2 + jc3 / 327270.;
        // Astronomical Algorithms pg. 144 (Longitude of the ascending node of the Moon's mean orbit)
        let om = 125.04452 - 1934.136261 * jc + 0.0020708 * jc2 + jc3 / 450000.;
        let xi_arr = [d, m, m_p, f, om];

        // Nutation in longitude and obliquity
        let (psi, eps) = Astro::calc_psi_eps(&xi_arr, jc);

        let k = 36000000.;
        let delta_psi = psi / k;
        let delta_eps = eps / k;

        // Astronomical Algorithms pg. 147 (22.3) (Mean obliquity of the ecliptic)
        let u = jm / 10.;
        let us = Astro::pow_series(u, 10);
        let e0 = 84381.448 - 4680.93 * us[0] - 1.55 * us[1] + 1999.25 * us[2]
            - 51.38 * us[3]
            - 249.67 * us[4]
            - 39.05 * us[5]
            + 7.12 * us[6]
            + 27.87 * us[7]
            + 5.79 * us[8]
            + 2.45 * us[9];
        // True obliquity of the ecliptic
        let e = (e0 / 3600. + delta_eps).to_radians();
        // Ecliptical longitude
        let l = lsum.to_degrees().cap_angle_360() + 180.;
        let l = l.cap_angle_360() + delta_psi + -20.4898 / (3600. * rsum);
        let l = l.to_radians();
        // Ecliptical latitude
        let b = -bsum.to_degrees();
        let b = b.to_radians();

        // Astronomical Algorithms pg. 93 (13.3)
        let ran = l.sin() * e.cos() - b.tan() * e.sin();
        let ra = ran.atan2(l.cos()).to_degrees().cap_angle_360();

        // Astronomical Algorithms pg. 93 (13.4)
        let dec = (b.sin() * e.cos() + b.cos() * e.sin() * l.sin()).asin();

        // Astronomical Algorithms pg. 88 (Mean sidereal time)
        let v0 = 280.46061837 + 360.98564736629 * j + 0.000387933 * jc2 - jc3 / 38710000.;
        // Apparent sidereal time
        let sid_time = v0.cap_angle_360() + delta_psi * e.cos();

        Self {
            dec,
            ra,
            sid_time,
            rsum,
            dra: 0.,
        }
    }

    fn calc_total(elems: &[(f64, f64, f64)], jm: f64) -> f64 {
        elems
            .iter()
            .fold(0., |acc, (i0, i1, i2)| acc + *i0 * (*i1 + *i2 * jm).cos())
    }

    fn calc_sum(elems: &[&[(f64, f64, f64)]], jms: &[f64]) -> f64 {
        elems.iter().enumerate().fold(0., |acc, (idx, xi)| {
            acc + Astro::calc_total(*xi, jms[1]) * jms[idx]
        }) / TEN_POW_EIGHT
    }

    fn calc_psi_eps(elems: &[f64; 5], jc: f64) -> (f64, f64) {
        // Astronomical Algorithms pg. 144 (Nutations in longitude and obliquity)
        SIN_COEFFICIENT.iter().enumerate().fold(
            (0., 0.),
            |acc, (idx, (sc0, sc1, sc2, sc3, sc4))| {
                let scs = [
                    *sc0 as f64,
                    *sc1 as f64,
                    *sc2 as f64,
                    *sc3 as f64,
                    *sc4 as f64,
                ];
                let xi_sum = elems
                    .iter()
                    .enumerate()
                    .fold(0., |acc, (idx, xi)| acc + *xi * scs[idx]);
                let xi_sum_rads = xi_sum.to_radians();
                // Nutation in longitude
                let psi = acc.0 + PE[idx].0 + jc * PE[idx].1 * xi_sum_rads.sin();
                // Nutation in obliquity
                let eps = acc.1 + PE[idx].2 + jc * PE[idx].3 * xi_sum_rads.cos();
                (psi, eps)
            },
        )
    }

    fn pow_series(val: f64, count: usize) -> Vec<f64> {
        let mut v = Vec::with_capacity(count);
        v.push(val);
        let mut curr = val;
        for _ in 1..count {
            curr *= val;
            v.push(curr);
        }
        v
    }
}

#[derive(Debug, Clone)]
pub struct AstroDay {
    astros: Vec<Astro>,
    julian_day: JulianDay,
}

impl AstroDay {
    pub fn new(julian_day: JulianDay) -> Self {
        let jd_val = f64::from(julian_day);
        let mut astros = Vec::new();
        astros.push(Astro::new(jd_val - 1.));
        astros.push(Astro::new(jd_val));
        astros.push(Astro::new(jd_val + 1.));

        Self { astros, julian_day }
    }
}

#[derive(Debug, Clone)]
pub struct TopAstroDay {
    astro_day: AstroDay,
    pub coords: Coordinates,
    astros: Vec<Astro>,
}

impl TopAstroDay {
    pub fn from_jd(julian_day: JulianDay, coords: Coordinates) -> Self {
        Self::from_ad(AstroDay::new(julian_day), coords)
    }

    fn from_ad(astro_day: AstroDay, coords: Coordinates) -> Self {
        let mut astros = Vec::new();

        for astro in astro_day.astros.iter() {
            // Astronomical Algorithms pg. 82
            let b_a = 0.99664719;
            let lat_rads = f64::from(coords.latitude).to_radians();
            let u = (b_a * lat_rads.tan()).atan();
            let elev = f64::from(coords.elevation);
            let p_sin_phi = b_a * u.sin() + elev / EARTH_RADIUS * lat_rads.sin();
            let p_cos_phi = u.cos() + elev / EARTH_RADIUS * lat_rads.cos();
            // Astronomical Algorithms pg. 279 (40.1)
            let earth_dist = 3600. * astro.rsum;
            let pi = (8.794 / earth_dist).to_radians();
            let hours = (astro.sid_time + f64::from(coords.longitude) - astro.ra)
                .cap_angle_360()
                .to_radians();
            // Astronomical Algorithms pg. 279 (40.2)
            let dra = -p_cos_phi * pi.sin() * hours.sin();
            let dra = dra.atan2(astro.dec.cos() - p_cos_phi * pi.sin() * hours.cos());
            // Astronomical Algorithms pg. 279 (40.3)
            let dec = (astro.dec.sin() - p_sin_phi * pi.sin()) * dra.cos();
            let dec = dec
                .atan2(astro.dec.cos() - p_cos_phi * pi.sin() * hours.cos())
                .to_degrees();

            let top_astro = Astro {
                ra: astro.ra + dra.to_degrees(),
                sid_time: astro.sid_time,
                dra,
                rsum: astro.rsum,
                dec,
            };

            astros.push(top_astro);
        }

        Self {
            astro_day,
            coords,
            astros,
        }
    }

    pub fn new_coords(&self, coords: Coordinates) -> Self {
        Self::from_ad(self.astro_day.clone(), coords)
    }

    pub fn astro(&self) -> &Astro {
        &self.astros[1]
    }

    pub fn prev_astro(&self) -> &Astro {
        &self.astros[0]
    }

    pub fn next_astro(&self) -> &Astro {
        &self.astros[2]
    }

    pub fn julian_day(&self) -> JulianDay {
        self.astro_day.julian_day
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use float_cmp::assert_approx_eq;

    use crate::geo::{coordinates::*, julian_day::*};

    use super::*;

    const EPSILON_TEST: f64 = 0.00000001;

    #[test]
    fn should_new_astro() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2023, 1, 25).unwrap();
        let julian_day = JulianDay::new(date, Gmt::new(-5.).unwrap());
        // Act
        let astro = Astro::new(f64::from(julian_day));
        // Assert
        assert_approx_eq!(f64, 307.28605197, astro.ra, epsilon = EPSILON_TEST);
        assert_approx_eq!(f64, 0., astro.dra, epsilon = EPSILON_TEST);
        assert_approx_eq!(f64, 0.98446346, astro.rsum, epsilon = EPSILON_TEST);
        assert_approx_eq!(f64, -0.33216269, astro.dec, epsilon = EPSILON_TEST);
        assert_approx_eq!(f64, 199.24752575, astro.sid_time, epsilon = EPSILON_TEST);
    }

    #[test]
    fn should_new_astro_day() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2023, 1, 25).unwrap();
        let julian_day = JulianDay::new(date, Gmt::new(-5.).unwrap());
        // Act
        let astro_day = AstroDay::new(julian_day);
        // Assert
        assert_eq!(3, astro_day.astros.len());
        assert_eq!(julian_day, astro_day.julian_day);
        assert_approx_eq!(
            f64,
            306.24007524,
            astro_day.astros[0].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(f64, 0., astro_day.astros[0].dra, epsilon = EPSILON_TEST);
        assert_approx_eq!(
            f64,
            0.98436500,
            astro_day.astros[0].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -0.33639010,
            astro_day.astros[0].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            198.26187838,
            astro_day.astros[0].sid_time,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            307.28605197,
            astro_day.astros[1].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(f64, 0., astro_day.astros[1].dra, epsilon = EPSILON_TEST);
        assert_approx_eq!(
            f64,
            0.98446346,
            astro_day.astros[1].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -0.33216269,
            astro_day.astros[1].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            199.24752575,
            astro_day.astros[1].sid_time,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            308.32865839,
            astro_day.astros[2].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(f64, 0., astro_day.astros[2].dra, epsilon = EPSILON_TEST);
        assert_approx_eq!(
            f64,
            0.98456559,
            astro_day.astros[2].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -0.32783394,
            astro_day.astros[2].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            200.23317312,
            astro_day.astros[2].sid_time,
            epsilon = EPSILON_TEST
        );
    }

    #[test]
    fn should_new_top_astro_day() {
        // Arrange
        let date = NaiveDate::from_ymd_opt(2023, 1, 25).unwrap();
        let julian_day = JulianDay::new(date, Gmt::new(-5.).unwrap());
        let astro_day = AstroDay::new(julian_day);
        let coords = Coordinates::new(
            Latitude::new(39.0181651).unwrap(),
            Longitude::new(-77.2085914).unwrap(),
            Elevation::new(0.).unwrap(),
        );
        // Act
        let top_astro_day = TopAstroDay::from_ad(astro_day, coords);
        // Assert
        assert_eq!(3, top_astro_day.astros.len());
        assert_eq!(coords, top_astro_day.coords);
        assert_eq!(julian_day, top_astro_day.astro_day.julian_day);
        assert_approx_eq!(
            f64,
            306.23989035,
            top_astro_day.astros[0].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -3.22693472e-6,
            top_astro_day.astros[0].dra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            0.98436500,
            top_astro_day.astros[0].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -19.27456501,
            top_astro_day.astros[0].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            198.26187838,
            top_astro_day.astros[0].sid_time,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            307.28586523,
            top_astro_day.astros[1].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -3.259248858e-6,
            top_astro_day.astros[1].dra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            0.98446346,
            top_astro_day.astros[1].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -19.03236237,
            top_astro_day.astros[1].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            199.24752575,
            top_astro_day.astros[1].sid_time,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            308.32846992,
            top_astro_day.astros[2].ra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -3.28930619e-6,
            top_astro_day.astros[2].dra,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            0.98456559,
            top_astro_day.astros[2].rsum,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            -18.78435333,
            top_astro_day.astros[2].dec,
            epsilon = EPSILON_TEST
        );
        assert_approx_eq!(
            f64,
            200.23317312,
            top_astro_day.astros[2].sid_time,
            epsilon = EPSILON_TEST
        );
    }
}
