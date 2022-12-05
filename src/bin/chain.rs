/**
 * Позволяет избежать привязки отправителя запроса к его получателю, давая шанс 
 * обработать запрос нескольким объектам. Связывает объекты получатели в цепочку 
 * и передает запрос вдоль этой цепочки, пока его не обработают.
 *
 * Применимость:
 * - когда есть более одного объекта, способного обработать запрос, причем
 *   настоящий обработчик заранее неизвестен и должен быть найден автоматически
 * - когда вы хотите отправить запрос одному из нескольких объектов, не указывая 
 *   явно, какому именно.
 * - когда набор объектов, способных обработать запрос, должен задаваться
 *   динамически.
**/

#[derive(Clone)]
enum Event {
    Validate,
    Save,
}
trait Handler {
    fn execute(&self, event: Event);
    fn set_next(&mut self, handler: Box<dyn Handler>);
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
    fn execute(&self, event: Event) {
        match event {
            Event::Validate => {
                println!("Validate document");
            }
            _ => {
                if let Some(ref handler) = self.handler {
                    handler.execute(event);
                }
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
    fn execute(&self, event: Event) {
        match event {
            Event::Save => {
                println!("Save document");
            }
            _ => {
                if let Some(ref handler) = self.handler {
                    handler.execute(event);
                }
            }
        }
    }
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.handler = Some(handler);
    }
}
struct Document {
    handler: Option<Box<dyn Handler>>,
}

impl Document {
    fn new() -> Self {
        Document {
            handler: Default::default(),
        }
    }
    fn set_handler(&mut self, handler: Box<dyn Handler>) {
        self.handler = Some(handler);
    }
    fn validate(&self) {
        if let Some(ref handler) = self.handler {
            handler.execute(Event::Validate);
        }
    }
    fn save(&self) {
        if let Some(ref handler) = self.handler {
            handler.execute(Event::Save);
        }
    }
}

fn main() {
    let mut document = Document::new();
    let mut handler = Box::new(Validator::new());
    handler.set_next(Box::new(Keeper::new()));

    document.set_handler(handler);
    document.validate();
    document.save();
}
