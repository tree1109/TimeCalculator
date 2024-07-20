use super::time::Time;

pub struct TimeInterval {
    start_time: Time,
    end_time: Time,
}

impl TimeInterval {
    pub fn new(start_time: Time, end_time: Time) -> Self {
        Self {
            start_time,
            end_time,
        }
    }

    pub fn get_start_time(&self) -> &Time {
        &self.start_time
    }

    pub fn get_end_time(&self) -> &Time {
        &self.end_time
    }

    pub fn get_time_interval_in_minutes(&self) -> u16 {
        self.start_time.get_difference_in_minutes(&self.end_time)
    }

    pub fn get_time_interval_string(&self) -> String {
        format!(
            "{} ~ {}",
            self.start_time.get_time_string(),
            self.end_time.get_time_string(),
        )
    }
}
