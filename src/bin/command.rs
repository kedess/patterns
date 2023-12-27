use std::rc::Rc;

/**
* Инкапсулирует запрос как объект, позволяя тем самым задавать параметры
* клиентов для обработки соответствующих запросов, ставить запросы в очередь
* или прокотолировать их, а также поддерживать отмену операций.
*
* Применимость:
* - когда нужно параметризовать объекты выполняемым действием
* - когда нужно определять, ставить в очередь и выполнять запросы в разное время
* - когда нужно поддерживать отмену операций
* - когда нужно поддерживать протоколирование изменений, чтобы их можно было
*   выполнить повторно после аварийной остановки системы
**/

struct BusinessLogic {}
impl BusinessLogic {
    fn new() -> Self {
        BusinessLogic {}
    }
    fn open(&self) {
        println!("Open transaction");
    }
    fn validate(&self) {
        println!("Validate transaction");
    }
    fn close(&self) {
        println!("Close transaction");
    }
}

trait Command {
    fn execute(&self);
}

struct OpenCommand {
    logic: Rc<BusinessLogic>,
}
impl OpenCommand {
    fn new(logic: Rc<BusinessLogic>) -> Self {
        OpenCommand { logic }
    }
}
impl Command for OpenCommand {
    fn execute(&self) {
        self.logic.open();
    }
}
struct ValidateCommand {
    logic: Rc<BusinessLogic>,
}
impl ValidateCommand {
    fn new(logic: Rc<BusinessLogic>) -> Self {
        ValidateCommand { logic }
    }
}
impl Command for ValidateCommand {
    fn execute(&self) {
        self.logic.validate();
    }
}
struct CloseCommand {
    logic: Rc<BusinessLogic>,
}
impl CloseCommand {
    fn new(logic: Rc<BusinessLogic>) -> Self {
        CloseCommand { logic }
    }
}
impl Command for CloseCommand {
    fn execute(&self) {
        self.logic.close();
    }
}

struct UserInvoker {
    open: OpenCommand,
    validate: ValidateCommand,
    close: CloseCommand,
}

impl UserInvoker {
    fn new(
        open: OpenCommand,
        validate: ValidateCommand,
        close: CloseCommand,
    ) -> Self {
        UserInvoker {
            open,
            validate,
            close,
        }
    }
    fn open(&self) {
        self.open.execute();
    }
    fn validate(&self) {
        self.validate.execute();
    }
    fn close(&self) {
        self.close.execute();
    }
}

fn main() {
    let logic = Rc::new(BusinessLogic::new());
    let user = UserInvoker::new(
        OpenCommand::new(logic.clone()),
        ValidateCommand::new(logic.clone()),
        CloseCommand::new(logic),
    );
    user.open();
    user.validate();
    user.close();
}
