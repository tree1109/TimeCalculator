use super::Time;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(13, 10).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(*time_interval.get_start_time().get_hours(), 12);
        assert_eq!(*time_interval.get_start_time().get_minutes(), 5);
        assert_eq!(*time_interval.get_end_time().get_hours(), 13);
        assert_eq!(*time_interval.get_end_time().get_minutes(), 10);
    }

    #[test]
    fn test_get_time_interval_in_minutes() {
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(13, 10).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_in_minutes(), 65);
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(11, 10).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_in_minutes(), 1385);
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(12, 5).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_in_minutes(), 0);
    }

    #[test]
    fn test_get_time_interval_string() {
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(13, 10).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_string(), "12:05 ~ 13:10");
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(11, 10).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_string(), "12:05 ~ 11:10");
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(12, 5).unwrap();
        let time_interval = TimeInterval::new(start_time, end_time);
        assert_eq!(time_interval.get_time_interval_string(), "12:05 ~ 12:05");
    }
}
