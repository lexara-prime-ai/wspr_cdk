use reqwest;
use scraper;

fn main() {
    let test_url = "https://www.flightglobal.com/news/";
    let response = reqwest::blocking::get(test_url).unwrap().text().unwrap();

    // Extract Information from HTML.
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("div.subSleeve>h2").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));

    // println!("{:?}", response);
}

// CSS selectors that uniquely identifies those items
