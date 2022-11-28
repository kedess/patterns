/*
* Шаблон «Команда» позволяет инкапсулировать действия в объекты. Ключевая идея — предоставить средства отделения клиента от получателя.
* Команда – это поведенческий паттерн проектирования, который превращает запросы в объекты, позволяя передавать их как аргументы при вызове методов,
* ставить запросы в очередь, логировать их, а также поддерживать отмену операций.
* В шаблоне «Команда» объект используется для инкапсуляции всей информации, необходимой для выполнения действия либо для его инициирования позднее.
* Информация включает в себя имя метода; объект, владеющий методом; значения параметров метода.
*
* Применимость:
* - когда вы хотите параметризовать объекты выполняемым действием.
* - когда вы хотите ставить операции в очередь, выполнять их по расписанию или передавать по сети.
* - когда вам нужна операция отмены (храним историю).
*/

struct System {}
impl System {
    fn new() -> Self {
        System {  }
    }
    fn start(&self) {
        println!("Start");
    }
    fn stop(&self) {
        println!("Stop");
    }
}

 trait Command {
    fn execute(&self);
 }

 struct StartCommand<'a>{
    system: &'a System
 }
 impl <'a> StartCommand<'a> {
    fn new(system: &'a System) -> Self {
        StartCommand { system }
    }
 }
 impl <'a> Command for StartCommand <'a> {
    fn execute(&self) {
        self.system.start();
    }
 }
 struct StopCommand<'a>{
    system: &'a System
 }
 impl <'a> StopCommand<'a> {
    fn new(system: &'a System) -> Self {
        StopCommand { system }
    }
 }
 impl <'a> Command for StopCommand <'a> {
    fn execute(&self) {
        self.system.stop();
    }
 }

 struct UserInvoker<'a> {
    start: StartCommand<'a>,
    stop: StopCommand<'a>
 }

 impl<'a> UserInvoker <'a> {
     fn new(start: StartCommand<'a>, stop: StopCommand<'a>) -> Self {
        UserInvoker { start, stop}
     }
     fn start_computer(&self){
        self.start.execute();
     }
     fn stop_computer(&self){
        self.stop.execute();
     }
 }
 
 fn main(){
    let system = System::new();
    let user = UserInvoker::new(StartCommand::new(&system), StopCommand::new(&system));
    user.start_computer();
    user.stop_computer();
 }