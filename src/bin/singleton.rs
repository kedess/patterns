/**
 * Одиночка (Singleton) – это порождающий паттерн проектирования, который гарантирует, что у класса есть только один экземпляр,
 * и предоставляет к нему глобальную точку доступа. Этот экземпляр создается внутри класса и предоставляется через статический метод.
 * Используется когда требуется глобальный доступ к экземпляру объекта или контроль над созданием
 * и инициализацией из любой части программы.
 *
 * Когда использовать:
 * - Когда в программе должен быть единственный экземпляр какого-то класса, доступный всем клиентам
 * - Когда вам хочется иметь больше контроля над глобальными переменными
**/
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

// Структура будет в модуле, поэтому напрямую создать ее нельзя, так как поля не публичные
pub struct Dictionary {
    data: HashMap<i32, i32>,
}
impl Dictionary {
    // Конструктор также будет скрыт в не модуля
    fn new() -> Self {
        Dictionary {
            data: Default::default(),
        }
    }
    pub fn insert(&mut self, key: i32, value: i32) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key: &i32) -> Option<&i32> {
        self.data.get(key)
    }
    pub fn get_instance() -> &'static Mutex<Dictionary> {
        static INSTANCE: OnceLock<Mutex<Dictionary>> = OnceLock::new();
        INSTANCE.get_or_init(|| Mutex::new(Dictionary::new()))
    }
}
fn main() {
    {
        let mut dict = Dictionary::get_instance().lock().unwrap();
        dict.insert(1, 1);
    }
    for _ in 0..10 {
        // Каждый раз мы получаем один и тот же объект
        let dict = Dictionary::get_instance().lock().unwrap();
        println!("{:?}", dict.get(&1));
    }
}
