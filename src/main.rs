use std::collections::HashMap;
use std::convert::Infallible;

use warp::{http::Uri, Filter, Rejection, Reply};
use warp::http::StatusCode;

use askama::Template;


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
		.and(warp::body::content_length_limit(1024 * 100))
		.and(get_input())
		.map(|input: String| {
			let data = Page::create(input);
			data.render().unwrap()
		})
		.map(|rendered| warp::reply::html(rendered));

	let routes = root.or(input_page).or(results).recover(bad_request);
	
	warp::serve(routes)
		.run(([127,0,0,1],3030))
	.await;
}

fn get_input() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::body::form().and_then(|mut content: HashMap<String, String> | async move {
        if let Some(input) = content.remove("input") {
            Ok(input)
        } else {
            Err(warp::reject::custom(EmptyBody))
        }
    })
}

async fn bad_request(err: Rejection) -> Result<impl Reply, Infallible> {
	println!("{:?}",err);
	if err.is_not_found() {
		 Ok(warp::reply::with_status("Неверный путь", StatusCode::NOT_FOUND))
	} else if let Some(_) = err.find::<EmptyBody>() {
		Ok(warp::reply::with_status("Отсутствует поле!", StatusCode::BAD_REQUEST))
	} else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
		Ok(warp::reply::with_status("Слишком длинный текст!", StatusCode::BAD_REQUEST))
	} else {
		Ok(warp::reply::with_status("Bad Request", StatusCode::BAD_REQUEST))
	 }
}

#[derive(Debug)]
struct EmptyBody;
impl warp::reject::Reject for EmptyBody {}

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
