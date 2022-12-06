/**
 * Шаблон «Строитель» предназначен для поиска решения проблемы антипаттерна
 * Telescoping constructor.
 * 
 * Шаблон позволяет создавать разные свойства объекта, избегая загрязнения 
 * конструктора (constructor pollution). Это полезно, когда у объекта может быть
 * несколько свойств. Или когда создание объекта состоит из большого количества
 * этапов.  
 * 
 * Когда использовать:
 * - когда у объекта может быть несколько свойств и когда нужно избежать 
 *   Telescoping constructor. Ключевое отличие от шаблона «Простая фабрика»: 
 *   он используется в одноэтапном создании, а «Строитель» — в многоэтапном.
 **/
#[derive(Debug)]
enum Color {
    Black,
    White,
}

#[derive(Debug)]
#[allow(unused)]
struct Car {
    name: String,
    color: Color,
}

struct CarBuilder {
    name: String,
    color: Color,
}

impl CarBuilder {
    fn new() -> Self {
        CarBuilder {
            name: "No name".to_string(),
            color: Color::White,
        }
    }
    fn build(self) -> Car {
        Car {
            name: self.name,
            color: self.color,
        }
    }
    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    fn paint(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

fn main() {
    let car = CarBuilder::new()
        .name("Lada Vesta")
        .paint(Color::Black)
        .build();
    println!("{:?}", car);
}
