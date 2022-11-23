/*
 * Шаблон определяет зависимость между объектами, чтобы при изменении состояния одного из них его «подчинённые» узнавали об этом.
 * В шаблоне «Наблюдатель» есть объект («субъект»), ведущий список своих «подчинённых» («наблюдателей») и автоматически уведомляющий их о любом изменении своего состояния,
 * обычно с помощью вызова одного из их методов.
 * В данной реализации для того чтобы отписаться нужно пересоздать элемент Subscriber
*/

use std::rc::{Rc, Weak};
use std::vec::Vec;
use std::cell::RefCell;

trait Publisher {
    fn add_subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>);
    fn notify(&mut self);
}

trait Subscriber {
    fn event(&mut self, discounts: Vec<String>);
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
                Some(subscriber) => subscriber.borrow_mut().event(vec!["Socks".to_string()]),
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
    products: Vec<String>
}

impl Person {
    fn new(name: &str) -> Self {
        Person { 
            name: name.to_string(),
            products: Vec::new()
        }
    }
}

impl Subscriber for Person {
    fn event(&mut self, discounts: Vec<String>) {
        println!("New Discount for {}", self.name);
        for item in discounts {
            self.products.push(item);
        }
        println!("Current products for {}: {:?}", self.name, self.products);
    }
}

fn main(){
    let mut shop = Shop::new();

    let person1 = Rc::new(RefCell::new(Person::new("Person number 1")));
    let person2 = Rc::new(RefCell::new(Person::new("Person number 2")));
    shop.add_subscriber(person1.clone());
    shop.add_subscriber(person2.clone());
    {
        let person3 = Rc::new(RefCell::new(Person::new("Person number 3")));
        shop.add_subscriber(person3.clone());
        shop.notify();
    }
    shop.notify();
    shop.notify();
}
