/**
 * Шаблон «Декоратор» позволяет во время выполнения динамически изменять
 * поведение объекта, обёртывая его в объект класса «декоратора».
 *
 * Шаблон «Декоратор» позволяет подключать к объекту дополнительное поведение
 * (статически или динамически), не влияя на поведение других объектов того же
 * класса. Шаблон часто используется для соблюдения принципа единственной
 * обязанности (Single Responsibility Principle), поскольку позволяет разделить
 * функциональность между классами для решения конкретных задач.
**/

trait Coffe {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}

struct SimpleCoffe {}
impl Coffe for SimpleCoffe {
    fn cost(&self) -> f32 {
        100_f32
    }
    fn description(&self) -> String {
        "Coffe".to_string()
    }
}

struct WrapperMilkCoffe {
    coffe: Box<dyn Coffe>,
}

impl WrapperMilkCoffe {
    fn new(coffe: Box<dyn Coffe>) -> Self {
        WrapperMilkCoffe { coffe }
    }
}

impl Coffe for WrapperMilkCoffe {
    fn cost(&self) -> f32 {
        self.coffe.cost() * 1.1
    }
    fn description(&self) -> String {
        format!("{} with milk", self.coffe.description())
    }
}

fn main() {
    let milk_coffe = WrapperMilkCoffe::new(Box::new(SimpleCoffe {}));
    println!(
        "Description: {}\nCost: {}",
        milk_coffe.description(),
        milk_coffe.cost()
    );
}
