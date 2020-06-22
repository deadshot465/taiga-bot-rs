use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};

#[derive(Deserialize, Serialize, Debug)]
struct Url {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Query {
    pub urls: Url
}

#[derive(Deserialize, Serialize, Debug)]
struct SearchResult {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<Query>
}

const ITEM_PER_PAGE: u8 = 10;

pub async fn get_image(keyword: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let token: &str = dotenv!("UNSPLASH_TOKEN");
    let client = reqwest::Client::new();
    let response = client.get(format!("https://api.unsplash.com/search/photos?client_id={}&query={}&page=1", token, keyword).as_str())
        .send()
        .await?;
    let data: SearchResult = response.json().await?;
    let (total, total_pages) = (data.total, data.total_pages);

    if total == 0 {
        return Ok(vec![]);
    }

    // Limit to the first 25% pages.
    let upper_page_limit = ((total_pages as f32) * 0.25_f32).ceil();
    let random_page_number = thread_rng().gen_range(0_u32, (upper_page_limit as u32) + 1_u32);
    let response = client.get(format!("https://api.unsplash.com/search/photos?client_id={}&query={}&page={}", token, keyword, random_page_number).as_str())
        .send()
        .await?;
    let data: SearchResult = response.json().await?;
    let modulo = data.total % (ITEM_PER_PAGE as u32);
    let mut item_no = 0_usize;
    {
        let mut rng = thread_rng();
        item_no = if random_page_number == total_pages {
            rng.gen_range(0_usize, modulo as usize)
        }
        else {
            rng.gen_range(0_usize, ITEM_PER_PAGE as usize)
        };
    }
    let link = &data.results[item_no].urls.regular;

    let response = client.get(link.as_str())
        .header("Accept", "image/jpeg")
        .header("Content-Type", "image/jpeg")
        .send()
        .await?;
    let image_data = response.bytes().await?.to_vec();
    Ok(image_data)
}