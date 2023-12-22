/**
 * Это способ делегирования логики создания объектов (instantiation logic)
 * дочерним классам.
 *
 * В классо-ориентированном программировании (class-based programming)
 * фабричным методом называют порождающий шаблон проектирования, использующий
 * генерирующие методы (factory method) для решения проблемы создания объектов
 * без указания для них конкретных классов. Объекты создаются посредством вызова
 * не конструктора, а генерирующего метода, определённого в интерфейсе и
 * реализованного дочерними классами либо реализованного в базовом классе и,
 * опционально, переопределённого (overridden) производными классами
 * (derived classes).
 *
 * Когда использовать:
 * - Этот шаблон полезен для каких-то общих обработок в классе, но требуемые
 *   подклассы динамически определяются в ходе выполнения (runtime). То есть
 *   когда клиент не знает, какой именно подкласс может ему понадобиться.
*/

trait Transport {
    fn deliver(&self);
}

struct Car {}
impl Car {
    fn new() -> Self {
        Car {}
    }
}
impl Transport for Car {
    fn deliver(&self) {
        println!("Delivery by car");
    }
}
struct Ship {}
impl Ship {
    fn new() -> Self {
        Ship {}
    }
}
impl Transport for Ship {
    fn deliver(&self) {
        println!("Delivery by ship");
    }
}

trait Logistics {
    // Фабричный метод
    fn make_transport(&self) -> Box<dyn Transport>;
}
struct RoadLogistics {}
impl RoadLogistics {
    fn new() -> Self {
        RoadLogistics {}
    }
}
impl Logistics for RoadLogistics {
    // Переопределяем фабричный метод
    fn make_transport(&self) -> Box<dyn Transport> {
        Box::new(Car::new())
    }
}
struct SeaLogistics {}
impl SeaLogistics {
    fn new() -> Self {
        SeaLogistics {}
    }
}
impl Logistics for SeaLogistics {
    // Переопределяем фабричный метод
    fn make_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship::new())
    }
}
fn main() {
    let logistic_list: Vec<Box<dyn Logistics>> = vec![
        Box::new(RoadLogistics::new()),
        Box::new(SeaLogistics::new()),
    ];
    for logistics in logistic_list {
        let transport = logistics.make_transport();
        transport.deliver();
    }
}
