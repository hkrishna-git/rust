fn print1(elements : &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print2(elements : &Vec<String>) {
    elements.iter().for_each(|e| println!("{}", e));
}

fn print3(elements: &Vec<String>) {
    elements
    .iter()
    .map(|e| format!("{} {}",e,e))
    .for_each(|e| println!("{}", e)); 
}

fn slicing(elements : &[String]) {
    for element in elements {
        println!("{}", element);
    }
}

fn truncate(elements : &mut Vec<String>) {
    elements.iter_mut().for_each(|e| e.truncate(1));
}

fn to_upper(elements : &Vec<String>) -> Vec<String> {
    elements
    .iter()
    .map(|e| e.to_uppercase())
    .collect()
}

fn explode(elements: &Vec<String>) -> Vec<Vec<String>> {
    elements
    .iter()
    .map(|e| e.chars().map(|c| c.to_string()).collect())
    .collect()
}

fn find_color_or(elements: &Vec<String>, search:&str, fallback:&str) -> String {
    elements
    .iter()
    .find(|e| e.contains(search))
    .map_or(String::from(fallback),
            |e| e.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("yellow")
    ];

    print1(&colors);
    println!("******************************************************************");
    print2(&colors);
    println!("******************************************************************");
    print3(&colors);
    println!("******************************************************************");
    slicing(&colors[1..3]);
    println!("******************************************************************");
    slicing(&colors);
    println!("******************************************************************");
    //truncate(&mut colors);
    //println!("{:#?}", colors);
    println!("******************************************************************");
    let uppercased = to_upper(&colors);
    println!("{:#?}", uppercased);
    println!("******************************************************************");
    let exploded = explode(&colors);
    println!("{:#?}", exploded);
    println!("******************************************************************");
    let found = find_color_or(&colors, "ue", "not found");
    println!("{}", found);
}
