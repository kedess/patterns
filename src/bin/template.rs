use std::path::Path;

/**
 * Шаблонный метод определяет основу алгоритма и позволяет подклассам переопределить некоторые шаги алгоритма, не изменяя его структуру в целом.
 * 
 * Применимость:
 * - когда нужно однократно использовать инвариантные части алгоритма, оставляя реализацию изменяющегося поведения на усмотрение подклассов
 * - когда нужно вычленить и локализовать в одном классе поведение, общее для всех подклассов, дабы избежать дублирование кода
**/
struct DocumentEntity {
}
trait Document {
    fn open(&self, path: &str) -> Option<DocumentEntity> {
        if Path::new(path).extension().unwrap() == self.allow_extension() {
            return Some(DocumentEntity { })
        }
        None
    }
    fn allow_extension(&self) -> &'static str;
}

struct DocumentWord {}
impl DocumentWord {
    fn new() -> Self {
        DocumentWord {  }
    }
}
impl Document for DocumentWord {
    fn allow_extension(&self) -> &'static str {
        "doc"
    }
}
struct DocumentPDF {}
impl DocumentPDF {
    fn new() -> Self {
        DocumentPDF {  }
    }
}
impl Document for DocumentPDF {
    fn allow_extension(&self) -> &'static str {
        "pdf"
    }
}

fn main(){
    let document = DocumentWord::new();
    if let Some(_) = document.open("/home/user/rules.doc"){
        println!("Document opened");
    }
    let document = DocumentPDF::new();
    if let Some(_) = document.open("/home/user/rules.doc"){
        println!("Document opened");
    }
}