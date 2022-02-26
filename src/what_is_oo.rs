use crate::logger;

pub fn what_is_oo() {
    let mut c = AveragedCollection::new(None);
    c.add(1);
    c.add(2);
    c.add(3);
    logger::info(&format!("average: {}", c.average()));
    c.add(4);
    let p = c.pop();
    logger::info(&format!("pop: {:?}", p));
    logger::info(&format!("average: {}", c.average()));
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(value: Option<Vec<i32>>) -> AveragedCollection {
        match value {
            Some(value) => AveragedCollection {
                list: value,
                average: 0.0,
            },
            None => AveragedCollection {
                list: vec![],
                average: 0.0,
            },
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn pop(&mut self) -> Option<i32>{
        let poped = self.list.pop();
        match poped {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
