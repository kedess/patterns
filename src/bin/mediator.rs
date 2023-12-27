/**
* Определяет объект, инкапсулирующий способ взаимодействия множества объектов.
* Посредник обеспечивает слабую связанность системы, избавляя объекты от
* необходимости явно ссылаться друг на друга и позволяя тем самым независимо
* изменять взаимодействие между ними.
*
* Когда использовать:
* - когда имеются объекты, связи между которыми сложны и четко определены.
*   Получающиеся при этом взаимозависимости не структурированы и трудны для
*   понимания.
* - когда нельзя повторно использовать объект, поскольку он обменивается
*   информацией со многими другими объектами.
* - когда поведение, распределенное между несколькими классами, должно
*   поддаваться настройке без порождения множества подклассов.
**/
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone, Copy, PartialEq)]
enum UserKind {
    Developer,
    TeamLead,
}
#[derive(Debug, Clone)]
enum Event {
    CreateUser,
    ChangeUser,
}
trait Colleague {
    fn send(&self, event: Event);
    fn recv(&self, event: Event);
    fn kind(&self) -> UserKind;
}

trait Mediator {
    fn send(&mut self, event: Event);
    fn register(&mut self, colleague: Rc<dyn Colleague>);
}
struct MediatorColleague {
    colleagues: Vec<Weak<dyn Colleague>>,
}

impl MediatorColleague {
    fn new() -> Self {
        MediatorColleague { colleagues: vec![] }
    }
}

impl Mediator for MediatorColleague {
    fn send(&mut self, event: Event) {
        let mut need = false;
        match event {
            Event::CreateUser => {
                for colleague in &self.colleagues {
                    match colleague.upgrade() {
                        Some(colleague) => {
                            if colleague.kind() == UserKind::TeamLead {
                                colleague.recv(event.clone());
                            }
                        }
                        None => need = true,
                    }
                }
            }
            Event::ChangeUser => {
                for colleague in &self.colleagues {
                    match colleague.upgrade() {
                        Some(colleague) => {
                            colleague.recv(event.clone());
                        }
                        None => need = true,
                    }
                }
            }
        }
        if need {
            self.colleagues.retain(|x| x.upgrade().is_some());
        }
    }
    fn register(&mut self, colleague: Rc<dyn Colleague>) {
        self.colleagues.push(Rc::downgrade(&colleague));
    }
}

struct User {
    kind: UserKind,
    mediator: Weak<RefCell<dyn Mediator>>,
}

impl User {
    fn new(kind: UserKind, mediator: Rc<RefCell<dyn Mediator>>) -> Self {
        User {
            kind,
            mediator: Rc::downgrade(&mediator),
        }
    }
}

impl Colleague for User {
    fn recv(&self, event: Event) {
        println!("Received message [{:?}].", event);
    }
    fn send(&self, event: Event) {
        if let Some(mediator) = self.mediator.upgrade() {
            (*mediator).borrow_mut().send(event);
        }
    }
    fn kind(&self) -> UserKind {
        self.kind
    }
}

fn main() {
    let mediator = Rc::new(RefCell::new(MediatorColleague::new()));

    let user1 = Rc::new(User::new(UserKind::TeamLead, mediator.clone()));
    (*mediator).borrow_mut().register(user1.clone());
    let user2 = Rc::new(User::new(UserKind::Developer, mediator.clone()));
    (*mediator).borrow_mut().register(user2.clone());
    let user3 = Rc::new(User::new(UserKind::Developer, mediator.clone()));
    (*mediator).borrow_mut().register(user3.clone());

    user2.send(Event::CreateUser);
    user1.send(Event::ChangeUser);
    user3.send(Event::CreateUser);
}
