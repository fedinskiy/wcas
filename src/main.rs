use warp::{http::Uri, Filter};
use askama::Template;
use std::collections::HashMap;


#[tokio::main]
async fn main() {
	let root = warp::path::end().map(|| {
		warp::redirect(Uri::from_static("counter"))
	});
	let counter=warp::path!("counter");

	let input_page = counter
		.and(warp::get())
		.map(|| {
			let data = Page::init();
			data.render().unwrap()
		})
		.map(|rendered| warp::reply::html(rendered));
		
	let results = counter
		.and(warp::post())
		.and(warp::body::form())
		.map(|mut content: HashMap<String, String> | {
			let data = Page::create(content.remove("input").unwrap());
			data.render().unwrap()
		})
		.map(|rendered| warp::reply::html(rendered));

	let routes = root.or(input_page).or(results);
	
	warp::serve(routes)
		.run(([127,0,0,1],3030))
	.await;
}


#[derive(Template)]
#[template(path="test.html")]
struct Page {
	was: String,
	letters: usize,
	length: usize,
}

impl Page {
	fn create(s:String)->Page {
		let mut total:usize=0;
		let mut letters:usize=0;
		s.chars()
			.for_each(|letter|{
				if !(letter=='\r'){
					total+=1;
				}
				if !letter.is_whitespace() {
					letters+=1;
				}
			});
		
		Page {
			letters: letters, 
			length: total,
			was: s
		}
	}
	fn init() -> Page {
		Page::create(String::from(""))
	}
}
