/**
 * Шаблонный метод определяет основу алгоритма и позволяет подклассам
 * переопределить некоторые шаги алгоритма, не изменяя его структуру в целом.
 *
 * Когда использовать:
 * - когда нужно однократно использовать инвариантные части алгоритма, оставляя
 *   реализацию изменяющегося поведения на усмотрение подклассов
 * - когда нужно вычленить и локализовать в одном классе поведение, общее для
 *   всех подклассов, дабы избежать дублирование кода
**/

trait TestingSystem {
    fn prepare(&self);
    fn finish(&self);
    fn run(&self) {
        self.prepare();
        println!("Run tests");
        self.finish();
    }
}

struct TestingSystemWin {}
impl TestingSystemWin {
    fn new() -> Self {
        TestingSystemWin {}
    }
}
impl TestingSystem for TestingSystemWin {
    fn prepare(&self) {
        println!("Download windows docker image");
    }
    fn finish(&self) {
        println!("Remome windows docker image");
    }
}

struct TestingSystemLinux {}
impl TestingSystemLinux {
    fn new() -> Self {
        TestingSystemLinux {}
    }
}
impl TestingSystem for TestingSystemLinux {
    fn prepare(&self) {
        println!("Download linux docker image");
    }
    fn finish(&self) {
        println!("Remome linux docker image");
    }
}

fn main() {
    let testing_system = Box::new(TestingSystemLinux::new());
    testing_system.run();
    let testing_system = Box::new(TestingSystemWin::new());
    testing_system.run();
}
