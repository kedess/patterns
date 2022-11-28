/**
 ** Шаблон определяет зависимость между объектами, чтобы при изменении состояния одного из них все зависящие от него оповещаются и автоматически обновляются.
 * Применимость:
 * - когда при модификации одного объекта требуется изменить другие и вы не знаете, сколько именно объектов нужно изменить.
 * - когда один объект должен оповещать других, не делая предположений об уведомляемых объектах. Другими словами, вы не хотите, чтобы
 *   объекты были тесно связанны между собой.
 * 
 * ! У данной реализации есть следующие проблемы:
 * ! - подписчик не может отписаться, ссылка на subscriber удалиться, но только во время следующего обновления publisher 
 **/

use std::rc::{Rc, Weak};
use std::vec::Vec;
use std::cell::RefCell;

enum Message {
    Shop(String),
    Storage((String, usize))
}

trait Publisher {
    fn add_subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>);
    fn notify(&mut self);
}

trait Subscriber {
    fn update(&mut self, msg: Message);
}

struct Shop {
    subscribers: Vec<Weak<RefCell<dyn Subscriber>>>
}

impl Shop {
    fn new() -> Self {
        Shop { subscribers: Vec::new() }
    }
}

impl Publisher for Shop {
    fn add_subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>) {
        self.subscribers.push(Rc::downgrade(&subscriber));
    }
    fn notify(&mut self){
        let mut need = false;
        for subscriber in &self.subscribers {
            match subscriber.upgrade(){
                Some(subscriber) => subscriber.borrow_mut().update(Message::Shop("New discounts".to_string())),
                None => need = true
            }
        }
        if need {
            self.subscribers.retain(|x| x.upgrade().is_some());
        }
    }
}

struct Storage {
    subscribers: Vec<Weak<RefCell<dyn Subscriber>>>
}

impl Storage {
    fn new() -> Self {
        Storage { subscribers: Vec::new() }
    }
}

impl Publisher for Storage {
    fn add_subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>) {
        self.subscribers.push(Rc::downgrade(&subscriber));
    }
    fn notify(&mut self){
        let mut need = false;
        for subscriber in &self.subscribers {
            match subscriber.upgrade(){
                Some(subscriber) => subscriber.borrow_mut().update(Message::Storage(("Product has arrived".to_string(), 100))),
                None => need = true
            }
        }
        if need {
            self.subscribers.retain(|x| x.upgrade().is_some());
        }
    }
}

struct Person {
    name: String,
    cnt: usize,
}

impl Person {
    fn new(name: &str) -> Self {
        Person { 
            name: name.to_string(),
            cnt: 0,
        }
    }
}

impl Subscriber for Person {
    fn update(&mut self, msg: Message) {
        println!("Recived [{}] message from publisher for user [{}]", self.cnt + 1, self.name);
        self.cnt += 1;
        match msg {
            Message::Shop(msg) => println!("{}", msg),
            Message::Storage((msg, _)) => println!("{}", msg)
        }
    }
}

fn main(){
    let mut shop = Shop::new();
    let mut storage = Storage::new();

    let person1 = Rc::new(RefCell::new(Person::new("Person number 1")));
    let person2 = Rc::new(RefCell::new(Person::new("Person number 2")));

    shop.add_subscriber(person1.clone());
    shop.add_subscriber(person2.clone());
    storage.add_subscriber(person2.clone());
    {
        let person3 = Rc::new(RefCell::new(Person::new("Person number 3")));
        shop.add_subscriber(person3.clone());
        storage.add_subscriber(person3.clone());
        shop.notify();
        storage.notify();
    }
    shop.notify();
    storage.notify();
}