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

struct TestingSystemUbuntu {}
impl TestingSystemUbuntu {
    fn new() -> Self {
        TestingSystemUbuntu {}
    }
}
impl TestingSystem for TestingSystemUbuntu {
    fn prepare(&self) {
        println!("Download ubuntu docker image");
    }
    fn finish(&self) {
        println!("Remome ubuntu docker image");
    }
}

struct TestingSystemDebian {}
impl TestingSystemDebian {
    fn new() -> Self {
        TestingSystemDebian {}
    }
}
impl TestingSystem for TestingSystemDebian {
    fn prepare(&self) {
        println!("Download debian docker image");
    }
    fn finish(&self) {
        println!("Remome debian docker image");
    }
}

fn main() {
    let testing_system = Box::new(TestingSystemDebian::new());
    testing_system.run();
    let testing_system = Box::new(TestingSystemUbuntu::new());
    testing_system.run();
}
