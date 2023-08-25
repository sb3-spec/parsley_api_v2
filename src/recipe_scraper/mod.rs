use scraper::{ElementRef, Html, Selector};
use std::string::FromUtf8Error;
use thiserror::Error;

use crate::models::Recipe;

pub async fn scrape_recipe(encoded_url: &str) -> Result<Recipe, ScraperError> {
    let url = urlencoding::decode(encoded_url)?;

    let res = reqwest::get(url.as_ref()).await?.text().await?;

    let html = Html::parse_document(&res);

    let meta_selector = Selector::parse("meta")?;
    let ul_selector = Selector::parse("ul")?;
    let ol_selector = Selector::parse("ol")?;
    let div_selector = Selector::parse("div")?;
    let p_selector = Selector::parse("p")?;

    let mut recipe_data = Recipe::default();

    let ingredient_filter = |ul: &ElementRef<'_>| -> bool {
        if ul
            .ancestors()
            .filter_map(ElementRef::wrap)
            .filter(|el| el.value().name() == "nav")
            .count()
            > 0
        {
            return false;
        }
        if let Some(parent) = ul.parent() {
            if let Some(element) = ElementRef::wrap(parent) {
                element.inner_html().to_lowercase().contains("ingredients")
            } else {
                false
            }
        } else {
            false
        }
    };

    let scrape_meta_data = |element: scraper::ElementRef<'_>| -> () {
        let element_content = element.value().attr("content");
        match element.value().attr("property") {
            Some("og:title") => {
                if let Some(data) = element_content {
                    recipe_data.title = data.to_owned();
                }
            }
            Some("og:description") => {
                if let Some(data) = element_content {
                    recipe_data.header = data.to_owned();
                }
            }
            Some("og:image") => {
                if let Some(data) = element_content {
                    recipe_data.image_url = data.to_owned();
                }
            }
            _ => {}
        }
    };

    let scrape_ingredients = |element: scraper::ElementRef<'_>| -> () {
        // I think I can safely assume that the ingredient list will be an unordered list
        if recipe_data.ingredients.len() > 0 {
            return;
        }

        let ingredients: Vec<String> = element
            .children()
            .filter_map(ElementRef::wrap)
            .filter(|element| element.value().name() == "li")
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect::<Vec<_>>();

        recipe_data.ingredients = ingredients;
    };

    let parse_step = |element: scraper::ElementRef<'_>| -> String {
        if let Some(text_node) = Html::parse_fragment(&element.inner_html())
            .select(&p_selector)
            .next()
        {
            text_node
                .text()
                .collect::<Vec<_>>()
                .join("")
                .replace("\n", "")
        } else {
            element.inner_html()
        }
    };

    let scrape_directions = |element: scraper::ElementRef| -> () {
        if recipe_data.steps.len() > 0 {
            return;
        }
        let directions = element
            .children()
            .filter_map(ElementRef::wrap)
            .filter(|element| element.value().name() == "li")
            .map(parse_step)
            .collect::<Vec<_>>();

        recipe_data.steps = directions;
    };

    let scrape_cook_times = |element: scraper::ElementRef| -> () {
        let cook_time_descriptors = ["prep", "cook", "total"];

        let formatted_cook_time_label = element
            .text()
            .collect::<Vec<_>>()
            .join("")
            .replace(" ", "")
            .replace("-", "")
            .replace(":", "")
            .to_lowercase();

        for descriptor in cook_time_descriptors {
            let text = format!("{}time", descriptor);
            if formatted_cook_time_label.starts_with(&text) {
                let Some(cook_time_value) = element
                    .next_siblings()
                    .filter_map(ElementRef::wrap)
                .map(|el| el.text().collect::<Vec<_>>().join("")).next() else {
                    return;
                };

                match descriptor {
                    "prep" => {
                        recipe_data.prep_time = cook_time_value;
                    }
                    "cook" => {
                        recipe_data.cook_time = cook_time_value;
                    }
                    "total" => {
                        recipe_data.total_time = cook_time_value;
                    }
                    _ => {}
                };
                break;
            };
        }
    };

    let scrape_servings = |element: scraper::ElementRef| -> () {
        if element.inner_html().to_lowercase().starts_with("servings") {
            let Some(serving_value_element) = element.next_siblings().filter_map(ElementRef::wrap).next() else {
                return;
            };
            recipe_data.servings = serving_value_element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .parse()
                .unwrap_or(1);
        };
    };

    html.select(&meta_selector).for_each(scrape_meta_data);

    html.select(&ul_selector)
        .filter(ingredient_filter)
        .for_each(scrape_ingredients);

    html.select(&ol_selector).for_each(scrape_directions);

    html.select(&div_selector).for_each(scrape_cook_times);

    html.select(&div_selector).for_each(scrape_servings);

    Ok(recipe_data)
}

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ScraperError(#[from] scraper::error::SelectorErrorKind<'static>),

    #[error(transparent)]
    UrlDecodingError(#[from] FromUtf8Error),
}
