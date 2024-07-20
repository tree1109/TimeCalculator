use super::time_interval::TimeInterval;

pub struct TimeLog {
    time_intervals: Vec<TimeInterval>,
}

impl TimeLog {
    pub fn new(time_intervals: Vec<TimeInterval>) -> Self {
        Self { time_intervals }
    }

    pub fn get_total_time_in_minutes(&self) -> u16 {
        self.time_intervals
            .iter()
            .fold(0, |acc, x| acc + x.get_time_interval_in_minutes())
    }

    pub fn get_total_time_string(&self) -> String {
        let total = self.get_total_time_in_minutes();
        let hours = total / 60;
        let minutes = total % 60;
        format!("{:02}:{:02}", hours, minutes)
    }

    pub fn get_time_log_strings(&self) -> String {
        let strings: Vec<String> = self
            .time_intervals
            .iter()
            .map(|x| x.get_time_interval_string())
            .collect();
        strings.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::super::time::Time;
    use super::*;

    fn get_time_intervals() -> Vec<TimeInterval> {
        vec![
            TimeInterval::new(Time::new(6, 0).unwrap(), Time::new(18, 30).unwrap()),
            TimeInterval::new(Time::new(8, 0).unwrap(), Time::new(12, 0).unwrap()),
            TimeInterval::new(Time::new(13, 0).unwrap(), Time::new(2, 0).unwrap()),
        ]
    }

    #[test]
    fn test_get_total_time_in_minutes() {
        let time_intervals = get_time_intervals();
        let time_log = TimeLog::new(time_intervals);
        assert_eq!(time_log.get_total_time_in_minutes(), 1770);
    }

    #[test]
    fn test_get_total_time_string() {
        let time_intervals = get_time_intervals();
        let time_log = TimeLog::new(time_intervals);
        assert_eq!(time_log.get_total_time_string(), "29:30");
    }

    #[test]
    fn test_get_time_log_strings() {
        let time_intervals = get_time_intervals();
        let time_log = TimeLog::new(time_intervals);
        assert_eq!(
            time_log.get_time_log_strings(),
            "06:00 ~ 18:30\n08:00 ~ 12:00\n13:00 ~ 02:00"
        );
    }
}
