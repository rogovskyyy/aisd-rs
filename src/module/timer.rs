use crate::time::{ Time, Duration };

pub struct Timer { }

impl Timer {
    #[allow(deprecated)]
    pub fn get_current_time() -> Time {
        Time::now()
    }

    pub fn get_timespan(a: Time, b: Time) -> String {
        format!("{}", ((b - a) as Duration).whole_milliseconds())
    }
}