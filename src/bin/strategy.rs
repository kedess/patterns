/*
 * Шаблон «Стратегия» позволяет переключаться между алгоритмами или стратегиями в зависимости от ситуации.
 * Шаблон «Стратегия» позволяет при выполнении выбирать поведение алгоритма.
 *
 * Пирменимость:
 * - когда вам нужно использовать разные вариации какого-то алгоритма внутри одного объекта.
 * - когда у вас есть множество похожих классов, отличающихся только некоторым поведением.
 * - когда вы не хотите обнажать детали реализации алгоритмов для других классов.
 * - когда различные вариации алгоритмов реализованы в виде развесистого условного оператора. Каждая ветка такого оператора представляет собой вариацию алгоритма.
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
    sorted.sort(&mut data);
    println!("{:?}", data);
    let mut data = vec![4, 67, 2, 7, 5];
    sorted.set_sorting(Box::new(SecondStrategySorting::new()));
    sorted.sort(&mut data);
    println!("{:?}", data);
}
