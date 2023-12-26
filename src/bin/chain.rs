/**
 * Шаблон «Цепочка ответственности» содержит исходный управляющий объект и ряд
 * обрабатывающих объектов. Каждый обрабатывающий объект содержит логику,
 * определяющую типы командных объектов, которые он может обрабатывать, а
 * остальные передаются по цепочке следующему обрабатывающему объекту.
 *
 * Шаблон «Цепочка ответственности» позволяет создавать цепочки объектов.
 * Запрос входит с одного конца цепочки и движется от объекта к объекту, пока
 * не будет найден подходящий обработчик.
 *
 * Применимость:
 * - когда есть более одного объекта, способного обработать запрос, причем
 *   настоящий обработчик заранее неизвестен и должен быть найден автоматически
 * - когда вы хотите отправить запрос одному из нескольких объектов, не указывая
 *   явно, какому именно.
 * - когда набор объектов, способных обработать запрос, должен задаваться
 *   динамически.
**/

trait Handler {
    fn execute(&self);
    fn set_next(&mut self, handler: Box<dyn Handler>);
}

struct Processing {
    handler: Option<Box<dyn Handler>>,
}
impl Processing {
    fn new() -> Self {
        Processing { handler: None }
    }
}

impl Handler for Processing {
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.handler = Some(handler);
    }
    fn execute(&self) {
        if let Some(ref handler) = self.handler {
            handler.execute();
        }
    }
}

struct Validator {
    handler: Option<Box<dyn Handler>>,
}
impl Validator {
    fn new() -> Self {
        Validator {
            handler: Default::default(),
        }
    }
}
impl Handler for Validator {
    fn execute(&self) {
        let is_validate = true;
        println!("Validate document");
        // Если бы проверка не прошла, то цепочка бы разорвалась
        if is_validate {
            if let Some(ref handler) = self.handler {
                handler.execute();
            }
        }
    }
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.handler = Some(handler);
    }
}
struct Keeper {
    handler: Option<Box<dyn Handler>>,
}
impl Keeper {
    fn new() -> Self {
        Keeper {
            handler: Default::default(),
        }
    }
}
impl Handler for Keeper {
    fn execute(&self) {
        println!("Save document");
    }
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.handler = Some(handler);
    }
}

fn main() {
    let mut processing = Processing::new();
    let mut validator = Box::new(Validator::new());
    let keeper = Box::new(Keeper::new());

    validator.set_next(keeper);
    processing.set_next(validator);

    processing.execute();
}
