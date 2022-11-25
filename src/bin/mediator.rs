/*
* Шаблон «Посредник» подразумевает добавление стороннего объекта («посредника») для управления взаимодействием между двумя объектами («коллегами»).
* Шаблон помогает уменьшить связанность (coupling) классов, общающихся друг с другом, ведь теперь они не должны знать о реализациях своих собеседников.
* Шаблон определяет объект, который инкапсулирует способ взаимодействия набора объектов.
* Здесь довольно легко добавить например проверки прав отсылки сообщений и тп
 */

use std::{rc::{Weak, Rc}, cell::RefCell};

trait Colleague {
    fn send(&self, msg: &str, to: u32);
    fn recv(&self, msg: &str, from: u32);
    fn id(&self) -> u32;
}

trait Mediator {
    fn send_msg(&mut self, from: u32, msg: &str, to: u32);
    fn add_colleague(&mut self, colleague: Rc<dyn Colleague>);
}
struct MediatorColleague{
    colleagues: Vec<Weak<dyn Colleague>>,
}

impl MediatorColleague {
    fn new() -> Self {
        MediatorColleague { colleagues: vec![]}
    }
}

impl Mediator for MediatorColleague {
    fn send_msg(&mut self, from: u32, msg: &str, to: u32) {
        let mut need = false;
        for colleague in &self.colleagues {
            match colleague.upgrade() {
                Some(colleague) => {
                    if colleague.id() == to {
                        colleague.recv(msg, from);
                    }
                }
                None => need = true
            }
        }
        if need {
            self.colleagues.retain(|x| x.upgrade().is_some());
        }
    }
    fn add_colleague(&mut self, colleague: Rc<dyn Colleague>) {
        self.colleagues.push(Rc::downgrade(&colleague));
    }
}

struct User {
    id: u32,
    mediator: Weak<RefCell<dyn Mediator>>
}

impl User {
    fn new(id: u32, mediator: Rc<RefCell<dyn Mediator>>) -> Self {
        User{id, mediator: Rc::downgrade(&mediator)}
    }
}

impl Colleague for User {
    fn recv(&self, msg: &str, from: u32) {
        println!("Received message from user [{}] to user [{}]: [{}].", from, self.id, msg);
    }
    fn send(&self, msg: &str, id: u32) {
        if let Some(mediator) = self.mediator.upgrade() {
            (*mediator).borrow_mut().send_msg(self.id, msg, id);
        }
    }
    fn id(&self) -> u32 {
        self.id
    }
}

fn main(){
    let mediator = Rc::new(RefCell::new(MediatorColleague::new()));

    let user1 = Rc::new(User::new(1, mediator.clone()));
    (*mediator).borrow_mut().add_colleague(user1.clone());
    let user2 = Rc::new(User::new(2, mediator.clone()));
    (*mediator).borrow_mut().add_colleague(user2.clone());
    let user3 = Rc::new(User::new(3, mediator.clone()));
    (*mediator).borrow_mut().add_colleague(user3.clone());
    

    user1.send("Hello", 3);
    user3.send("Hi", 1);
}