/*
 * Шаблон «Компоновщик» позволяет клиентам обрабатывать отдельные объекты в
 * едином порядке.
 *
 * Шаблон «Компоновщик» описывает общий порядок обработки группы объектов,
 * словно это одиночный экземпляр объекта. Суть шаблона — компонование объектов
 * в древовидную структуру для представления иерархии от частного к целому.
 * Шаблон позволяет клиентам одинаково обращаться к отдельным объектам и к
 * группам объектов.
 *
 * Когда применять:
 * - Когда вам нужно представить древовидную структуру объектов
 * - Когда клиенты должны единообразно трактовать простые и составные объекты
*/

trait Component {
    fn weight(&self) -> u32;
}
trait CompositeComponent: Component {
    fn add_cargo(&mut self, cargo: Box<dyn Component>);
}

struct Wood {}
impl Wood {
    fn new() -> Wood {
        Wood {}
    }
}
impl Component for Wood {
    fn weight(&self) -> u32 {
        320
    }
}
struct Car {}
impl Car {
    fn new() -> Self {
        Car {}
    }
}
impl Component for Car {
    fn weight(&self) -> u32 {
        2200
    }
}

struct SeaContainer {
    items: Vec<Box<dyn Component>>,
}
impl SeaContainer {
    fn new() -> Self {
        SeaContainer { items: vec![] }
    }
}
impl CompositeComponent for SeaContainer {
    fn add_cargo(&mut self, cargo: Box<dyn Component>) {
        self.items.push(cargo);
    }
}
impl Component for SeaContainer {
    fn weight(&self) -> u32 {
        self.items.iter().map(|item| item.weight()).sum()
    }
}
struct Tanker {
    items: Vec<Box<dyn Component>>,
}
impl Tanker {
    fn new() -> Self {
        Tanker { items: vec![] }
    }
}
impl CompositeComponent for Tanker {
    fn add_cargo(&mut self, cargo: Box<dyn Component>) {
        self.items.push(cargo);
    }
}
impl Component for Tanker {
    fn weight(&self) -> u32 {
        self.items.iter().map(|item| item.weight()).sum()
    }
}

fn main() {
    /*
     * Не важно что это за объект у любого объекта, простого или состовного
     * мы можем запросить вес
     */
    let mut container = Box::new(SeaContainer::new());
    container.add_cargo(Box::new(Wood::new()));
    container.add_cargo(Box::new(Wood::new()));
    container.add_cargo(Box::new(Car::new()));

    let mut container2 = Box::new(SeaContainer::new());
    container2.add_cargo(Box::new(Wood::new()));
    container2.add_cargo(Box::new(Car::new()));
    container2.add_cargo(Box::new(Car::new()));

    let car = Box::new(Car::new());

    println!(
        "{} {} {}",
        container.weight(),
        container2.weight(),
        car.weight()
    );

    let mut tanker = Tanker::new();
    tanker.add_cargo(container);
    tanker.add_cargo(container2);
    tanker.add_cargo(car);
    println!("{}", tanker.weight());
}
