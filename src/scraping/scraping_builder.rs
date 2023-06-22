use scraper::Html;

pub struct ScraperBuiler{}

impl ScraperBuiler {
    pub async fn subcreation_weekly_affix() -> Result<String, reqwest::Error>{
        let url = "https://mplus.subcreation.net";
        let response = reqwest::get(url).await?;
        let raw_html_string = response.text().await?;
        let document = Html::parse_document(&raw_html_string);
        let class_selector = scraper::Selector::parse(".col-12.py-3").unwrap();
        let class = document.select(&class_selector);

        let mut cropped: String = String::new();
        for value in class {
            let info: Vec<_> = value.text().collect();
            let mut text = String::from(info[1]);
            text = text.replace('"', "");
            cropped = text[1..].to_string();
        }
        Ok(cropped)
    }
}