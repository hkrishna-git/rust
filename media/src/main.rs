mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let audiobook = Media::Audiobook{
        title:String::from("An audiobook")
    };

    let movie = Media::Movie{
        title:String::from("A movie"),
        director:String::from("A director")
    };

    audiobook.print();
    movie.print();

    println!("{}", audiobook.description());
    println!("{}", movie.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);

    println!("{:#?}", catalog);
}
