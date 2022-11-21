/*
 * Шаблон «Стратегия» позволяет переключаться между алгоритмами или стратегиями в зависимости от ситуации.
 */

trait Sorting {
    fn sort(&self, data: &mut [i32]);
}

struct FirstStrategySorting {}

impl FirstStrategySorting {
    fn new() -> Self {
        FirstStrategySorting {  }
    }
}

impl Sorting for FirstStrategySorting {
    fn sort(&self, data: &mut [i32]) {
        println!("Used first strategy");
        data.sort();
    }
}

struct SecondStrategySorting {}

impl SecondStrategySorting {
    fn new() -> Self {
        SecondStrategySorting {  }
    }
}

impl Sorting for SecondStrategySorting {
    fn sort(&self, data: &mut [i32]){
        println!("Used second strategy");
        data.sort()
    }
}

struct Sorted {
    sorting: Box<dyn Sorting>
}

impl Sorted {
    fn new(sorting: Box<dyn Sorting>) -> Self {
        Sorted { sorting }
    }
    fn sort(&self, data: &mut [i32]) {
        self.sorting.sort(data);
    }
    fn set_sorting(&mut self, sorting: Box<dyn Sorting>){
        self.sorting = sorting;
    }
}

fn main() {
    let mut sorted = Sorted::new(Box::new(FirstStrategySorting::new()));
    let mut data = vec![4, 67, 2, 7, 5];
    sorted.sort(&mut data[..]);
    println!("{:?}", data);
    let mut data = vec![4, 67, 2, 7, 5];
    sorted.set_sorting(Box::new(SecondStrategySorting::new()));
    sorted.sort(&mut data[..]);
    println!("{:?}", data);
}
