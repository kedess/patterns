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
#[allow(unused)]
enum TransmissionKind {
    Auto,
    Manual,
}
#[allow(unused)]
enum EngineKind {
    Gasoline,
    Diesel,
}
#[allow(unused)]
enum NavigationKind {
    Gps,
    Glonas,
}
#[allow(unused)]
struct Car {
    transmission: TransmissionKind,
    engine: EngineKind,
    navigation: Option<NavigationKind>,
}

struct CarBuilder {
    transmission: Option<TransmissionKind>,
    engine: Option<EngineKind>,
    navigation: Option<NavigationKind>,
}

impl CarBuilder {
    fn new() -> Self {
        CarBuilder {
            transmission: None,
            engine: None,
            navigation: None,
        }
    }
    fn build(mut self) -> Option<Car> {
        // Проверка построения объекта, только навигатор опционален
        if self.engine.is_some() && self.transmission.is_some() {
            return Some(Car {
                transmission: self.transmission.unwrap(),
                engine: self.engine.unwrap(),
                navigation: self.navigation.take(),
            });
        }
        None
    }
    fn transmission(mut self, transmission: TransmissionKind) -> Self {
        self.transmission.replace(transmission);
        self
    }
    fn engine(mut self, engine: EngineKind) -> Self {
        self.engine.replace(engine);
        self
    }
    fn navigation(mut self, navigation: NavigationKind) -> Self {
        self.navigation.replace(navigation);
        self
    }
}

fn main() {
    if let Some(_car) = CarBuilder::new()
        .engine(EngineKind::Gasoline)
        .transmission(TransmissionKind::Auto)
        .navigation(NavigationKind::Glonas)
        .build()
    {
        println!("The car was built successfully")
    }
}
