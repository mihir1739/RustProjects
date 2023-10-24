use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}
fn main() {
    let article = Article{
        article: String::from("Working With JSON in Rust"),
        author: String::from("Mihir"),
        paragraph: vec![
            Paragraph{
                name:String::from("First Sentence")
            },
            Paragraph{
                name:String::from("Body pf the Paragraph")
            },
            Paragraph{
                name:String::from("End of the Paragraph.")
            }
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("JSON is {}",json)
}
