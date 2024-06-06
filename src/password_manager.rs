use std::collections::HashMap;

struct Locked;
struct Unlocked;
struct PasswordManager<State = Locked> {
    master_password: String,
    passwords: HashMap<String, String>,
    // state: State, // this consumes space, whereas PhantomData<T> is zero-sized
    state: std::marker::PhantomData<State>,
}

impl<State> PasswordManager<State> {
    pub fn version(&self) {
        todo!()
    }

    pub fn encryption(&self) {
        todo!()
    }
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_password: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_password: self.master_password,
            passwords: self.passwords,
            state: std::marker::PhantomData::<Locked>,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

impl PasswordManager {
    pub fn new(master_password: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_password,
            passwords: HashMap::new(),
            state: std::marker::PhantomData::<Unlocked>,
        }
    }
}

fn main() {
    let manager = PasswordManager::new("My password".to_owned());
    let manager = manager.lock();
    let mut manager = manager.unlock("My password".to_owned());

    manager.add_password("test".to_owned(), "test".to_owned());
}
