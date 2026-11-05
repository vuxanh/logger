use chrono::{DateTime, Utc};

pub struct LogDateTime;

impl LogDateTime {
    pub fn now() -> DateTime<Utc> {
        if cfg!(test) {
            DateTime::from_timestamp_nanos(0)
        } else {
            Utc::now()
        }
    }
}

#[cfg(test)]
pub struct AfterTest {
    pub callback: Box<dyn Fn() -> Result<(), std::io::Error>>,
}

#[cfg(test)]
impl Drop for AfterTest {
    fn drop(&mut self) {
        println!("running clean up..");
        (self.callback)().expect("coudn't run clean up!");
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! after_test {
    ($body:block) => {
        let _after_test = $crate::utils::AfterTest {
            callback: Box::new(move || $body),
        };
    };
}
