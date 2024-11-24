use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Clock {
    gmt: u64,
    h: Option<u64>,
    m: Option<u64>,
    hour: Option<String>,
    minute: Option<String>,
    numbers: [String; 60],
}
impl Clock {
    pub fn new(gmt_hour: u64, gmt_minute: u64) -> Self {
        Clock {
            gmt: gmt_hour * 60 + gmt_minute,
            h: None,
            m: None,
            hour: None,
            minute: None,
            numbers: [
                "Zero".to_string(),
                "One".to_string(),
                "Two".to_string(),
                "Three".to_string(),
                "Four".to_string(),
                "Five".to_string(),
                "Six".to_string(),
                "Seven".to_string(),
                "Eight".to_string(),
                "Nine".to_string(),
                "Ten".to_string(),
                "Eleven".to_string(),
                "Twelve".to_string(),
                "Thirteen".to_string(),
                "Fourteen".to_string(),
                "Fifteen".to_string(),
                "Sixteen".to_string(),
                "Seventeen".to_string(),
                "Eighteen".to_string(),
                "Nineteen".to_string(),
                "Twenty".to_string(),
                "Twenty-One".to_string(),
                "Twenty-Two".to_string(),
                "Twenty-Three".to_string(),
                "Twenty-Four".to_string(),
                "Twenty-Five".to_string(),
                "Twenty-Six".to_string(),
                "Twenty-Seven".to_string(),
                "Twenty-Eight".to_string(),
                "Twenty-Nine".to_string(),
                "Thirty".to_string(),
                "Thirty-One".to_string(),
                "Thirty-Two".to_string(),
                "Thirty-Three".to_string(),
                "Thirty-Four".to_string(),
                "Thirty-Five".to_string(),
                "Thirty-Six".to_string(),
                "Thirty-Seven".to_string(),
                "Thirty-Eight".to_string(),
                "Thirty-Nine".to_string(),
                "Forty".to_string(),
                "Forty-One".to_string(),
                "Forty-Two".to_string(),
                "Forty-Three".to_string(),
                "Forty-Four".to_string(),
                "Forty-Five".to_string(),
                "Forty-Six".to_string(),
                "Forty-Seven".to_string(),
                "Forty-Eight".to_string(),
                "Forty-Nine".to_string(),
                "Fifty".to_string(),
                "Fifty-One".to_string(),
                "Fifty-Two".to_string(),
                "Fifty-Three".to_string(),
                "Fifty-Four".to_string(),
                "Fifty-Five".to_string(),
                "Fifty-Six".to_string(),
                "Fifty-Seven".to_string(),
                "Fifty-Eight".to_string(),
                "Fifty-Nine".to_string(),
            ],
        }
    }
    pub fn refresh(&mut self) {
        let now = SystemTime::now();
        let seconds_since_unix_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut minutes = seconds_since_unix_epoch / 60;
        minutes -= self.gmt; // converting to local time
        let mut hours = minutes / 60;
        minutes = minutes - hours * 60;
        hours = hours % 12;
        self.h = Some(hours);
        self.m = Some(minutes);
        self.to_string();
    }
    pub fn to_string(&mut self) {
        self.hour = match self.h {
            Some(0) => Some("Twelve".to_string()),
            Some(1..12) => Some(self.numbers[self.h.unwrap() as usize].clone()),
            _ => None,
        };
        self.minute = match self.m {
            Some(0) => Some("".to_string()),
            Some(1..60) => Some(self.numbers[self.m.unwrap() as usize].clone()),
            _ => None,
        };
    }
    pub fn get_hour(&self) -> String {
        self.hour.clone().unwrap_or("HH".to_string())
    }
    pub fn get_minute(&self) -> String {
        self.minute.clone().unwrap_or("MM".to_string())
    }
}
