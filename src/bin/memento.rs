/*
* Шаблон «Хранитель» фиксирует и хранит текущее состояние объекта, чтобы оно легко восстанавливалось.
* Шаблон «Хранитель» позволяет восстанавливать объект в его предыдущем состоянии (отмена через откат — undo via rollback).
*
* Применимость:
* - когда вам нужно сохранять мгновенные снимки состояния объекта (или его части), чтобы впоследствии объект можно было восстановить в том же состоянии.
* - когда прямое получение состояния объекта раскрывает приватные детали его реализации, нарушая инкапсуляцию.
 */
struct MementoArticle {
    state: String
}

impl MementoArticle {
    fn new(state: &str) -> Self {
        MementoArticle { state: state.to_string() }
    }
    fn get_state(&mut self) -> &str {
        &self.state
    }
}

struct Article {
    content: Option<String>,
    mementos: Vec<MementoArticle>
}

impl Article {
    fn new() -> Self {
        Article { content: None, mementos: vec![]}
    }
    fn undo(&mut self) {
        match self.mementos.pop() {
            Some(mut memento) => self.content = Some(memento.get_state().to_string()),
            None => self.content = None
        }
    }
    fn get_content(&self) -> Option<&str> {
        if let Some(ref content) = self.content {
            return Some(content)
        }
        None
    }
    fn set_content(&mut self, content: &str) {
        if let Some(ref content) = self.content {
            self.mementos.push(MementoArticle::new(content));
        }
        self.content = Some(content.to_string());
    }
}

fn main(){
    let mut article = Article::new();
    println!("{:?}", article.get_content());
    article.set_content("First content");
    println!("{:?}", article.get_content());
    article.set_content("Second content");
    println!("{:?}", article.get_content());
    article.undo();
    println!("{:?}", article.get_content());
    article.undo();
    println!("{:?}", article.get_content());
    article.undo();
    println!("{:?}", article.get_content());
    article.set_content("Third content");
    println!("{:?}", article.get_content());
    article.undo();
    println!("{:?}", article.get_content());
}