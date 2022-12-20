/*
 * Шаблон «Компоновщик» позволяет клиентам обрабатывать отдельные объекты в 
 * едином порядке.
 *
 * Шаблон «Компоновщик» описывает общий порядок обработки группы объектов, 
 * словно это одиночный экземпляр объекта. Суть шаблона — компонование объектов
 * в древовидную структуру для представления иерархии от частного к целому.
 * Шаблон позволяет клиентам одинаково обращаться к отдельным объектам и к 
 * группам объектов.
*/

trait Cargo {
    fn cost(&self) -> u32;
}

struct Wood {}
impl Wood {
    fn new() -> Wood {
        Wood {}
    }
}
impl Cargo for Wood {
    fn cost(&self) -> u32 {
        320
    }
}
struct Car {}
impl Car {
    fn new() -> Self {
        Car {}
    }
}
impl Cargo for Car {
    fn cost(&self) -> u32 {
        2200
    }
}

struct Tanker {
    items: Vec<Box<dyn Cargo>>,
}
impl Tanker {
    fn new() -> Self {
        Tanker { items: vec![] }
    }
    fn add_cargo(&mut self, cargo: Box<dyn Cargo>) {
        self.items.push(cargo);
    }
    fn cost(&self) -> u32 {
        self.items.iter().map(|item| item.cost()).sum()
    }
}
fn main() {
    let mut tanker = Tanker::new();
    tanker.add_cargo(Box::new(Wood::new()));
    tanker.add_cargo(Box::new(Wood::new()));
    tanker.add_cargo(Box::new(Car::new()));
    println!("{}", tanker.cost());
}
