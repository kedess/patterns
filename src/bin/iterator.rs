/**
* Представляет способ последовательного доступа ко всем элементам составного
* объекта, не раскрывая его внутреннего представления.
*
* Когда использовать:
* - когда нужен доступ к содержимому агрегированных объектов без раскрытия их
*   внутреннего представления.
* - когда нужна поддержка нескольких активных обходов одного и того же 
*   агрегированного объекта.
* - когда нужно представление единообразного интерфейса с целью обхода различных
*   агрегированных структур.
**/

#[derive(Clone)]
pub struct User {
    id: usize,
    name: String,
}
impl User {
    fn new(id: usize, name: &str) -> Self {
        User {
            id,
            name: name.to_string(),
        }
    }
}
struct Colleagues {
    users: Vec<User>,
}

impl Colleagues {
    fn new() -> Self {
        Colleagues { users: vec![] }
    }
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn iterator(&self) -> ColleaguesIter {
        ColleaguesIter {
            pos: 0,
            users: &self.users,
        }
    }
}

pub struct ColleaguesIter<'a> {
    pos: usize,
    users: &'a [User],
}

impl<'a> Iterator for ColleaguesIter<'a> {
    type Item = &'a User;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.users.len() {
            return None;
        }
        self.pos += 1;
        Some(&(self.users[self.pos - 1]))
    }
}
fn main() {
    let mut colleagues = Colleagues::new();
    colleagues.add_user(User::new(1, "John"));
    colleagues.add_user(User::new(2, "Smith"));
    colleagues.add_user(User::new(1, "Kate"));
    for user in colleagues.iterator() {
        println!("{} {}", user.id, user.name);
    }
}
