use warp::Filter;
use askama::Template;


#[tokio::main]
async fn main() {
	let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

	let html = warp::path!("counter")
		.map(|| {
			let data = Page::init();
			data.render().unwrap()
		})
		.map(|rendered| warp::reply::html(rendered));

	let routes = warp::get().and(hello).or(html);
	
	
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
		Page {
			letters: s.len(),
			length: s.len(),
			was: s
		}
	}
	fn init() -> Page {
		Page::create(String::from(""))
	}
}
