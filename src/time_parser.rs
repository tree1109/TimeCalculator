use crate::Time;
use crate::TimeInterval;
use regex::Regex;

pub fn get_time_from_string(time: &str) -> Time {
    let time: Vec<&str> = time.split(":").collect();
    let hours: u16 = time[0].parse::<u16>().unwrap();
    let minutes: u16 = time[1].parse::<u16>().unwrap();
    Time::new(hours, minutes).unwrap()
}

pub fn get_time_interval_from_line(line: &str) -> TimeInterval {
    let pattern: Regex = Regex::new(r"\d{1,2}:\d{2}").unwrap();
    let times: Vec<&str> = pattern.find_iter(line).map(|x| x.as_str()).collect();
    let start_time: Time = get_time_from_string(times[0]);
    let end_time: Time = get_time_from_string(times[1]);
    TimeInterval::new(start_time, end_time)
}

pub fn get_time_interval_from_lines(lines: Vec<&str>) -> Vec<TimeInterval> {
    lines
        .iter()
        .map(|x| get_time_interval_from_line(x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Time;
    use crate::TimeInterval;
    use crate::TimeLog;

    #[test]
    fn test_get_time_from_string() {
        let time: Time = get_time_from_string("12:34");
        assert_eq!(*time.get_hours(), 12);
        assert_eq!(*time.get_minutes(), 34);
        let time: Time = get_time_from_string("01:23");
        assert_eq!(*time.get_hours(), 1);
        assert_eq!(*time.get_minutes(), 23);
        let time: Time = get_time_from_string("09:44");
        assert_eq!(*time.get_hours(), 9);
        assert_eq!(*time.get_minutes(), 44);
    }

    #[test]
    fn test_get_time_interval_from_line() {
        let line: &str = "12:34 - 23:45";
        let time_interval: TimeInterval = get_time_interval_from_line(line);
        assert_eq!(*time_interval.get_start_time().get_hours(), 12);
        assert_eq!(*time_interval.get_start_time().get_minutes(), 34);
        assert_eq!(*time_interval.get_end_time().get_hours(), 23);
        assert_eq!(*time_interval.get_end_time().get_minutes(), 45);
        let line: &str = "01:23 - 04:56";
        let time_interval: TimeInterval = get_time_interval_from_line(line);
        assert_eq!(*time_interval.get_start_time().get_hours(), 1);
        assert_eq!(*time_interval.get_start_time().get_minutes(), 23);
        assert_eq!(*time_interval.get_end_time().get_hours(), 4);
        assert_eq!(*time_interval.get_end_time().get_minutes(), 56);
        let line: &str = "09:44 - 12:34";
        let time_interval: TimeInterval = get_time_interval_from_line(line);
        assert_eq!(*time_interval.get_start_time().get_hours(), 9);
        assert_eq!(*time_interval.get_start_time().get_minutes(), 44);
        assert_eq!(*time_interval.get_end_time().get_hours(), 12);
        assert_eq!(*time_interval.get_end_time().get_minutes(), 34);
    }

    #[test]
    fn test_get_time_intervals_from_lines() {
        let lines: Vec<&str> = vec!["1. From 12:10 to 15:10", "2. From 6:00 to 8:05"];
        let time_intervals: Vec<TimeInterval> = get_time_interval_from_lines(lines);
        assert_eq!(time_intervals.len(), 2);
        let time_log: TimeLog = TimeLog::new(time_intervals);
        assert_eq!(time_log.get_total_time_in_minutes(), 305);
        assert_eq!(time_log.get_total_time_string(), "05:05");
        let lines: Vec<&str> = vec![
            "1. From 1:10 to 6:10",
            "2. From 12:00 to 0:05",
            "3. From 12:40 to 13:40",
            "4. From 8:50 to 23:50",
        ];
        let time_intervals: Vec<TimeInterval> = get_time_interval_from_lines(lines);
        assert_eq!(time_intervals.len(), 4);
        let time_log: TimeLog = TimeLog::new(time_intervals);
        assert_eq!(time_log.get_total_time_in_minutes(), 1985);
        assert_eq!(time_log.get_total_time_string(), "33:05");
    }
}
