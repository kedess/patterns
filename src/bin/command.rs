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

struct SimulationSystem {}
impl SimulationSystem {
    fn new() -> Self {
        SimulationSystem {}
    }
    fn start(&self) {
        println!("Command: start simulation system");
    }
    fn stop(&self) {
        println!("Command: stop simulation system");
    }
}

struct StorageSystem {}
impl StorageSystem {
    fn new() -> Self {
        StorageSystem {}
    }
    fn save(&self) {
        println!("Command: save all results.");
    }
}

trait Command {
    fn execute(&self);
}

struct StartCommand<'a> {
    system: &'a SimulationSystem,
}
impl<'a> StartCommand<'a> {
    fn new(system: &'a SimulationSystem) -> Self {
        StartCommand { system }
    }
}
impl<'a> Command for StartCommand<'a> {
    fn execute(&self) {
        self.system.start();
    }
}
struct StopCommand<'a> {
    system: &'a SimulationSystem,
}
impl<'a> StopCommand<'a> {
    fn new(system: &'a SimulationSystem) -> Self {
        StopCommand { system }
    }
}
impl<'a> Command for StopCommand<'a> {
    fn execute(&self) {
        self.system.stop();
    }
}

struct SaveCommand<'a> {
    storage: &'a StorageSystem,
}
impl<'a> SaveCommand<'a> {
    fn new(storage: &'a StorageSystem) -> Self {
        SaveCommand { storage }
    }
}
impl<'a> Command for SaveCommand<'a> {
    fn execute(&self) {
        self.storage.save();
    }
}

struct UserInvoker<'a> {
    start: StartCommand<'a>,
    save: SaveCommand<'a>,
    stop: StopCommand<'a>,
}

impl<'a> UserInvoker<'a> {
    fn new(
        start: StartCommand<'a>,
        save: SaveCommand<'a>,
        stop: StopCommand<'a>,
    ) -> Self {
        UserInvoker { start, save, stop }
    }
    fn start(&self) {
        self.start.execute();
    }
    fn save(&self) {
        self.save.execute();
    }
    fn stop(&self) {
        self.stop.execute();
    }
}

fn main() {
    let storage = StorageSystem::new();
    let system = SimulationSystem::new();
    let user = UserInvoker::new(
        StartCommand::new(&system),
        SaveCommand::new(&storage),
        StopCommand::new(&system),
    );
    user.start();
    user.save();
    user.stop();
}
