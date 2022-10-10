#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    is_active: bool,
    sign_in_count: u32,
}

impl User {
    pub fn new(first_name: &str, last_name: &str) -> User {
        User {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            is_active: true,
            sign_in_count: 1,
        }
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    pub fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let person = User::new("yulin", "chen");
    let (f, l) = person.to_tuple();
    println!("{} {}", f, l);
}
