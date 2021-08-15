use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {

    pub fn new() -> School {
        School {
            grades: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let mut ls = match self.grades.get(&grade) {
            Some(ls) => ls.to_owned(),
            None => vec!()
        };
        ls.push(student.to_string());
        self.grades.insert(grade, ls);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades:Vec<u32> = self.grades.keys().map(|g| g.clone()).collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut ls = match self.grades.get(&grade) {
            Some(ls) => ls.clone(),
            None => vec!()
        };
        ls.sort();
        ls
    }

}
