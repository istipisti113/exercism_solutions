use std::collections::{HashMap};

pub struct School {
    grades : HashMap<u32, Vec::<String>>
}

impl  School  {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(grade_vec) = self.grades.get_mut(&grade){
            grade_vec.push(String::from(student));
        } else {
            self.grades.insert(grade, vec![String::from(student)]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().map(|x|*x).collect::<Vec<u32>>()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(grade_vec) = self.grades.get(&grade) {
            grade_vec.clone()
        } else {
            vec![]
        }
    }
}
