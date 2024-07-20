use std::fmt::format;

struct Time {
    hours: u16,
    minutes: u16,
}

impl Time {
    fn new(hours: u16, minutes: u16) -> Option<Self> {
        match Self::is_valid_time(&hours, &minutes) {
            true => Some(Self { hours, minutes }),
            false => None,
        }
    }

    fn get_hours(&self) -> &u16 {
        &self.hours
    }

    fn get_minutes(&self) -> &u16 {
        &self.minutes
    }

    fn set_hours(&mut self, hours: u16) {
        self.hours = hours;
    }
    fn set_minutes(&mut self, minutes: u16) {
        self.minutes = minutes;
    }

    fn is_valid_time(hours: &u16, minutes: &u16) -> bool {
        *hours < 24 && *minutes < 60
    }

    fn total_minutes(&self) -> u16 {
        self.hours * 60 + self.minutes
    }

    fn get_difference_in_minutes(&self, end_time: &Self) -> u16 {
        let mut ent_time_minutes = end_time.total_minutes();
        if (end_time.get_hours() < &self.hours) {
            ent_time_minutes += 24 * 60;
        }
        ent_time_minutes - self.total_minutes()
    }

    fn get_time_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

struct TimeInterval {
    start_time: Time,
    end_time: Time,
}

impl TimeInterval {
    fn new(start_time: Time, end_time: Time) -> Self {
        Self {
            start_time,
            end_time,
        }
    }

    fn get_start_time(&self) -> &Time {
        &self.start_time
    }

    fn get_end_time(&self) -> &Time {
        &self.end_time
    }

    fn get_time_interval_in_minutes(&self) -> u16 {
        self.start_time.get_difference_in_minutes(&self.end_time)
    }

    fn get_time_interval_string(&self) -> String {
        format!(
            "{} ~ {}",
            self.start_time.get_time_string(),
            self.end_time.get_time_string(),
        )
    }
}

struct TimeLog {
    time_intervals: Vec<TimeInterval>,
}

impl TimeLog {
    fn new(time_intervals: Vec<TimeInterval>) -> Self {
        Self { time_intervals }
    }

    fn get_total_time_in_minutes(&self) -> u16 {
        self.time_intervals
            .iter()
            .fold(0, |acc, x| acc + x.get_time_interval_in_minutes())
    }

    fn get_total_time_string(&self) -> String {
        let total = self.get_total_time_in_minutes();
        let hours = total / 60;
        let minutes = total % 60;
        format!("{:02}:{:02}", hours, minutes)
    }

    fn get_time_log_strings(&self) -> String {
        self.time_intervals
            .iter()
            .map(|x| x.get_time_interval_string())
            .reduce(|mut acc, x| {
                acc.push_str(&x);
                acc
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time = Time::new(12, 5).unwrap();
        assert_eq!(*time.get_hours(), 12);
        assert_eq!(*time.get_minutes(), 5);
        let time = Time::new(24, 2);
        assert!(time.is_none());
        let time = Time::new(12, 68);
        assert!(time.is_none());
        let time = Time::new(25, 99);
        assert!(time.is_none());
    }

    #[test]
    fn test_total_minutes() {
        let time = Time::new(12, 5).unwrap();
        assert_eq!(time.total_minutes(), 725);
        let time = Time::new(16, 48).unwrap();
        assert_eq!(time.total_minutes(), 1008);
        let time = Time::new(12, 30).unwrap();
        assert_eq!(time.total_minutes(), 750);
    }

    #[test]
    fn test_get_difference_in_minutes() {
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(13, 10).unwrap();
        assert_eq!(start_time.get_difference_in_minutes(&end_time), 65);
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(11, 10).unwrap();
        assert_eq!(start_time.get_difference_in_minutes(&end_time), 1385);
        let start_time = Time::new(12, 5).unwrap();
        let end_time = Time::new(12, 5).unwrap();
        assert_eq!(start_time.get_difference_in_minutes(&end_time), 0);
    }

    #[test]
    fn test_get_time_string() {
        let time = Time::new(12, 5).unwrap();
        assert_eq!(time.get_time_string(), "12:05");
        let time = Time::new(16, 48).unwrap();
        assert_eq!(time.get_time_string(), "16:48");
        let time = Time::new(3, 3).unwrap();
        assert_eq!(time.get_time_string(), "03:03");
    }
}
