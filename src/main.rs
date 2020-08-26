use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

fn get_num_cases() -> i32 {
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://www.temple.edu/life-temple/health-wellness/coronavirus-planning-safe-return/university-communication/active-covid-19-cases-temple-university").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());


    // Look for amount of total cases
    for element in document.find(Attr("class", "row_3 col_4").descendant(Name("p"))) {
        let num_cases = element.text().parse::<i32>().unwrap();
        return num_cases;
    }

    return 0;

}

fn main() {
    println!("{}", get_num_cases());
}

