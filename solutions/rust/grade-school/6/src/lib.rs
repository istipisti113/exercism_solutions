use std::collections::{HashMap};

pub struct School {
    students : HashMap<String, u32>
}

impl  School  {
    pub fn new() -> School {
        School { students: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.students.keys().map(|x|x.as_ref()).collect::<Vec<&str>>();
        if !students.contains(&student){
            self.students.insert(String::from(student), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.values().map(|x|*x).collect::<Vec<u32>>()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.students.iter().filter(|x|x.1==&grade).map(|x|String::from(x.0.clone())).collect::<Vec<String>>()
    }
}
