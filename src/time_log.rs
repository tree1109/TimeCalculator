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
