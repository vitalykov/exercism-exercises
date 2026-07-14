// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

const EARTH_PERIOD: f64 = 31557600.0;
const MERCURY_PERIOD: f64 = 0.2408467 * EARTH_PERIOD;
const VENUS_PERIOD: f64 = 0.61519726 * EARTH_PERIOD;
const MARS_PERIOD: f64 = 1.8808158 * EARTH_PERIOD;
const JUPITER_PERIOD: f64 = 11.862615 * EARTH_PERIOD;
const SATURN_PERIOD: f64 = 29.447498 * EARTH_PERIOD;
const URANUS_PERIOD: f64 = 84.016846 * EARTH_PERIOD;
const NEPTURE_PERIOD: f64 = 164.79132 * EARTH_PERIOD;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / MERCURY_PERIOD
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / VENUS_PERIOD
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / EARTH_PERIOD
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / MARS_PERIOD
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / JUPITER_PERIOD
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SATURN_PERIOD
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / URANUS_PERIOD
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / NEPTURE_PERIOD
    }
}
