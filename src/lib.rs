use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use markdown::{to_html_with_options, Options};


#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct ContentItem {
    pub name: String,
    pub status: String,
}

impl ContentItem {
    fn to_md(&self) -> String {
        format!("| {} | {} |", self.name, self.status)
    }
}

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Content {
    pub title: String,
    pub name_header: String,
    pub status_header: String,
    pub data: Vec<ContentItem>,
}

impl Content {
    fn to_md(&self) -> String {
        let header = format!(
            "## {} \r\n| {} | {} |\r\n| --- | --- |\r\n",
            self.title, self.name_header, self.status_header);

        let content: Vec<String> = self.data.iter()
            .map(|d| d.to_md())
            .collect();

        header + &content.join("\r\n")
    }
}

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct NowContent {
    pub movies: Content,
    pub books: Content,

}

impl NowContent {
    fn to_md(&self) -> String {
       format!("{}\r\n\r\n{}", &self.movies.to_md(), &self.books.to_md())
    }

    fn to_html(&self) -> String {
        to_html_with_options(&self.to_md(), &Options::gfm()).expect("")
    }
}

fn fetch_content(api: &String) -> FnResult<Vec<ContentItem>> {
    let req = HttpRequest{
        url: (&api).to_string(),
        method: Some("GET".to_string()),
        headers: BTreeMap::new(),
    };

    let res = http::request::<()>(&req, None)?;
    Ok(res.json::<Vec<ContentItem>>()?)
}



#[host_fn("extism:host/user")]
extern "ExtismHost" {
   fn toast(content: String);
   fn loading(show: String);
}

pub fn fetch_movie() -> FnResult<Content> {
    let api = String::from("https://api.bbki.ng/movies");
    let res = fetch_content(&api);
    let content = Content {
        title: String::from("电影"),
        name_header: String::from("名称"),
        status_header: String::from("状态"),
        data: res?,
    };
    Ok(content)
}

pub fn fetch_book() -> FnResult<Content> {
    let api = String::from("https://api.bbki.ng/books");
    let res = fetch_content(&api);
    let content = Content {
        title: String::from("阅读"),
        name_header: String::from("名称"),
        status_header: String::from("状态"),
        data: res?,
    };
    Ok(content)
}

#[plugin_fn]
pub fn now() -> FnResult<String> {
    unsafe {
        let _ = loading("true".to_string());
    };
    let movies = fetch_movie();
    let books = fetch_book(); 

    let now = NowContent {
        movies: movies?,
        books: books?,
    };

    unsafe {
        let _ = loading("false".to_string());
    };

    Ok(now.to_html())
}
