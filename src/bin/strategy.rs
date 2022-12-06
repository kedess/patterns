/**
 * Шаблон «Стратегия» позволяет переключаться между алгоритмами или стратегиями
 * в зависимости от ситуации.
 * Шаблон «Стратегия» позволяет при выполнении выбирать поведение алгоритма.
 *
 * Пирменимость:
 * - когда имеется много родственных классов, отличающихся только поведением
 * - когда вам нужно иметь несколько разных вариантов алгоритмов
 * - когда в алгоритме содержаться данные, о которых клиент не должен знать
 * - когда в классе определено много поведений, что представлено большим
 *   количество условных операторов
 **/

trait Sorting {
    fn sort(&self, data: &mut DataSet);
}

struct FastSorting {}

impl FastSorting {
    fn new() -> Self {
        FastSorting {}
    }
}

impl Sorting for FastSorting {
    fn sort(&self, obj: &mut DataSet) {
        println!("Used fast sorting");
        obj.data.sort();
    }
}

struct RegularSorting {}

impl RegularSorting {
    #[allow(unused)]
    fn new() -> Self {
        RegularSorting {}
    }
}

impl Sorting for RegularSorting {
    fn sort(&self, obj: &mut DataSet) {
        println!("Used regular sorting, but less memory is used");
        obj.data.sort()
    }
}

struct DataSet {
    data: Vec<i32>,
    sorting: Option<Box<dyn Sorting>>,
}

impl DataSet {
    fn new(data: Vec<i32>, sorting: Box<dyn Sorting>) -> Self {
        DataSet {
            data,
            sorting: Some(sorting),
        }
    }
    fn sort(&mut self) {
        let sorting = self.sorting.take().unwrap();
        sorting.sort(self);
        self.sorting = Some(sorting);
    }
    #[allow(unused)]
    fn set_sorting(&mut self, sorting: Box<dyn Sorting>) {
        self.sorting = Some(sorting);
    }
}

fn main() {
    let mut data_set =
        DataSet::new(vec![4, 67, 2, 7, 5], Box::new(FastSorting::new()));
    data_set.sort();
    println!("{:?}", data_set.data);
}