use std::cell::RefCell;
/**
 *  Шаблон определяет зависимость между объектами, чтобы при изменении состояния
 *  одного из них все зависящие от него оповещаются и автоматически обновляются.
 *
 * Применимость:
 * - когда при модификации одного объекта требуется изменить другие и вы не
 *   знаете, сколько именно объектов нужно изменить.
 * - когда один объект должен оповещать других, не делая предположений об
 *   уведомляемых объектах. Другими словами, вы не хотите, чтобы объекты были
 *   тесно связанны между собой.
 *
 * ! У данной реализации есть следующие проблемы:
 * ! - подписчик не может отписаться, ссылка на subscriber удалиться, но только
 * !   во время следующего обновления publisher
 **/
use std::rc::Rc;
use std::vec::Vec;

trait Publisher {
    fn subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>);
    fn unsubscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>);
    fn notify(&mut self);
}

trait Subscriber {
    fn update(&mut self, msg: String);
}

struct Shop {
    subscribers: Vec<Rc<RefCell<dyn Subscriber>>>,
}

impl Shop {
    fn new() -> Self {
        Shop {
            subscribers: Vec::new(),
        }
    }
}

impl Publisher for Shop {
    fn subscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>) {
        self.subscribers.push(subscriber.clone());
    }
    fn unsubscriber(&mut self, subscriber: Rc<RefCell<dyn Subscriber>>) {
        let mut idx = -1;
        for (i, sub) in self.subscribers.iter().enumerate() {
            if sub.as_ptr() == subscriber.as_ptr() {
                idx = i as i32;
                break;
            }
        }
        if idx != -1 {
            self.subscribers.remove(idx as usize);
        }
    }
    fn notify(&mut self) {
        for subscriber in &self.subscribers {
            subscriber
                .borrow_mut()
                .update("Hello from Shop".to_string());
        }
    }
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Self {
        Person {
            name: name.to_string(),
        }
    }
}

impl Subscriber for Person {
    fn update(&mut self, msg: String) {
        println!(
            "Recived message [{}] from publisher for user [{}]",
            msg, self.name
        );
    }
}

fn main() {
    let mut shop = Shop::new();

    let person1 = Rc::new(RefCell::new(Person::new("Person 1")));
    let person2 = Rc::new(RefCell::new(Person::new("Person 2")));
    let person3 = Rc::new(RefCell::new(Person::new("Person 3")));

    shop.subscriber(person1.clone());
    shop.subscriber(person2.clone());
    shop.subscriber(person3.clone());
    shop.notify();

    shop.unsubscriber(person1);
    shop.notify();
}
