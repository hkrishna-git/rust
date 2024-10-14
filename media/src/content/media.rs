#[derive(Debug)]
pub enum Media {
    Book {title:String, author:String},
    Movie {title:String, director:String},
    Audiobook {title:String}
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book {title, author} => {
                format!("Book : {}, author : {}", title, author)
            }
            Media::Movie {title, director} => {
                format!("Movie : {}, author : {}", title, director)
            }
            Media::Audiobook {title} => {
                format!("Audiobook : {}", title)
            }
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }
}
