/*
* Шаблон итератор — это способ доступа к элементам объекта без раскрытия базового представления.
*/
#[derive(Clone)]
pub struct User {
    id: usize,
    name: String,
}
impl User {
    fn new(id: usize, name: &str) -> Self {
        User { id, name: name.to_string() }
    }
}
struct Colleagues {
    users: Vec<User>
}

impl Colleagues {
    fn new() -> Self {
        Colleagues { users: vec![] }
    }
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn iterator(&self) -> ColleaguesIter {
        ColleaguesIter { pos: 0, users: &self.users }
    }
}

pub struct ColleaguesIter<'a> {
    pos: usize,
    users: &'a[User]
}

impl<'a> Iterator for ColleaguesIter<'a> {
    type Item = &'a User;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.users.len() {
            return None;
        }
        self.pos += 1;
        return Some(&(self.users[self.pos - 1]));
    }
}
fn main(){
    let mut colleagues = Colleagues::new();
    colleagues.add_user(User::new(1, "John"));
    colleagues.add_user(User::new(2, "Smith"));
    colleagues.add_user(User::new(1, "Kate"));
    for user in colleagues.iterator() {
        println!("{} {}", user.id, user.name);
    }
}