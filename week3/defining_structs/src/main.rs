
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
    email: String,
    phone: u64,
    active: bool,
}

impl Person {
    fn new(first_name: String, last_name: String, age: Option<u8>, email: String, phone: u64) -> Self {
        Self {
            first_name,
            last_name,
            age,
            email,
            phone,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(email: &String) -> Option<&str> {
        let bytes = email.as_bytes();
        // if the '@' character is not found, the function returns None
        for (i, &item) in bytes.iter().enumerate() {
            if item == b'@' {
                return Some(&email[..i]);
            }
        }
        None
    }
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

fn get_full_name(person: &Person) -> String {
    format!("{} {}", person.first_name.to_string(), person.last_name.to_string())
}

fn main() {
    let johnd = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: Some(30),
        email: String::from("john.doe@gmail.com"),
        phone: 3239972356,
        active: true,
    } ;

    println!("{:?}", johnd);

    println!("\nName: {} \nage: {:?} \nemail {} \nphone {} \nactive {}", 
            get_full_name(&johnd), Some(johnd.age), johnd.email, johnd.phone, johnd.active);

    let mut jane = Person::new(
        String::from("Jane"), 
        String::from("Doe"), 
        None, 
        String::from("jane.doe@gmail.com"),
        3239972356,
    );

    println!("\nName: {} \nage: {:?} \nemail {} \nphone {} \nactive {}", 
            get_full_name(&jane), Some(jane.age), jane.email, jane.phone, jane.active);

    println!("Jane (username: {}) account status is now deactivated", 
            Person::from_email(&jane.email).unwrap());
    jane.deactivate();
    println!("Jane account status is : {:?}", jane.active);
    jane.update_email("jane.bond@goodlife.com".to_string());
    println!("Jane new email is : {}", jane.email);

}