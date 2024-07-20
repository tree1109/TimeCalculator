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

    fn get_difference_in_minutes(&self, end_time: Self) -> u16 {
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
