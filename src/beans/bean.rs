use chrono::naive::NaiveDate;

pub enum BeanRoast {
    Light(i8),
    Medium(i8),
    Dark(i8),
}

impl BeanRoast {
    fn get_temp(&self) -> i8 {
        match self {
            BeanRoast::Dark(t) |
            BeanRoast::Medium(t) |
            BeanRoast::Light(t) => *t
        }
    }
}

pub struct Bean {
    roast: BeanRoast,
    roast_date: NaiveDate,
    source: String, //TODO: Location type?
}

impl Bean {
    pub fn new(roast: BeanRoast, roast_date: NaiveDate, source: String) -> Self {
        Bean {roast, roast_date, source}
    }

    pub fn reordered(&mut self, date: Option<NaiveDate>) {
        if let Some(date) = date {
            self.roast_date = date;
        } else {
            self.roast_date = chrono::offset::Local::now().date_naive();
        }
    }

    pub fn optimal_temperature(&self) -> i8 {
        self.roast.get_temp()
    }
}
