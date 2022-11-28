/**
* Не нарушая инкапсуляции, фиксирует и выносит за пределы объекта его внутреннее состояние так, чтобы позднее можно было восстановить в нем объект.
*
* Применимость:
* - необходимо сохранить мгновенный снимок состояния объекта (или его части), чтобы впоследствии объект можно было восстановить в том же состоянии
* - прямое получение этого состояния раскрывает детали реализации и нарушает инкапсуляцию объекта
**/

struct MementoArticle {
    state: String
}

struct Article {
    content: String,
}

impl Article {
    fn new() -> Self {
        Article { content: Default::default()}
    }
    fn set_memento(&mut self, memento: MementoArticle) {
        self.content = memento.state;
    }
    fn get_memento(&self) -> MementoArticle {
        MementoArticle { state: self.content.clone() }
    }
    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
    fn get_content(&self) -> &str {
        &self.content
    }
}

fn main(){
    let mut article = Article::new();
    article.set_content("First content");
    println!("{:?}", article.get_content());

    let memento = article.get_memento();

    article.set_content("Second content");
    println!("{:?}", article.get_content());
    article.set_memento(memento);
    println!("{:?}", article.get_content());
}