use std::collections::HashMap;
use std::convert::Infallible;

use warp::http::StatusCode;
use warp::{http::Response, http::Uri, Filter, Rejection, Reply};

use askama::Template;

static FAVICON_FILE: &'static [u8] = include_bytes!("calculator.svg");

#[tokio::main]
async fn main() {
	let root = warp::path::end().map(|| warp::redirect(Uri::from_static("counter")));
	let counter = warp::path!("counter");

	let favicon = warp::path!("favicon.svg")
		.and(warp::get())
		.map(|| Response::builder().body(FAVICON_FILE));

	let input_page = counter
		.and(warp::get())
		.map(|| {
			let data = Page::init();
			data.render().unwrap()
		})
		.map(|rendered| warp::reply::html(rendered));

	let lowered_results = warp::path!("lower")
		.and(warp::post())
		.and(warp::body::content_length_limit(1024 * 100))
		.and(get_input())
		.map(|input: String| {
			let data = Page::lower(input);
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

	let routes = root
		.or(favicon)
		.or(input_page)
		.or(results)
		.or(lowered_results)
		.recover(bad_request);

	println!("Starting the application");
	warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn get_input() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
	warp::body::form().and_then(|mut content: HashMap<String, String>| async move {
		if let Some(input) = content.remove("input") {
			Ok(input)
		} else {
			Err(warp::reject::custom(EmptyBody))
		}
	})
}

async fn bad_request(err: Rejection) -> Result<impl Reply, Infallible> {
	println!("{:?}", err);
	if err.is_not_found() {
		Ok(warp::reply::with_status(
			"Неверный путь",
			StatusCode::NOT_FOUND,
		))
	} else if let Some(_) = err.find::<EmptyBody>() {
		Ok(warp::reply::with_status(
			"Отсутствует поле!",
			StatusCode::BAD_REQUEST,
		))
	} else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
		Ok(warp::reply::with_status(
			"Слишком длинный текст!",
			StatusCode::BAD_REQUEST,
		))
	} else {
		Ok(warp::reply::with_status(
			"Bad Request",
			StatusCode::BAD_REQUEST,
		))
	}
}

#[derive(Debug)]
struct EmptyBody;
impl warp::reject::Reject for EmptyBody {}

#[derive(Template)]
#[template(path = "test.html")]
struct Page {
	text: String,
	letters: usize,
	length: usize,
}

impl Page {
	fn create(s: String) -> Page {
		let mut total: usize = 0;
		let mut letters: usize = 0;
		s.chars().for_each(|letter| {
			if !(letter == '\r') {
				total += 1;
			}
			if !letter.is_whitespace() {
				letters += 1;
			}
		});
		Page {
			letters: letters,
			length: total,
			text: s,
		}
	}

	fn lower(s: String) -> Page {
		Page {
			letters: 0,
			length: 0,
			text: s.to_lowercase(),
		}
	}

	fn init() -> Page {
		Page::create(String::from(""))
	}
}

#[cfg(test)]
mod tests {
	use super::Page;

	#[test]
	fn smoke() {
		let page = Page::create(String::from("wq"));
		assert_eq!(page.letters, 2);
		assert_eq!(page.length, 2);
		assert_eq!(page.text, "wq");
	}

	#[test]
	fn unicode() {
		let page = Page::create(String::from("Привет, příteli!"));
		assert_eq!(page.letters, 15);
		assert_eq!(page.length, 16);
		assert_eq!(page.text, "Привет, příteli!");
	}

	#[test]
	fn unicode_lower() {
		let page = Page::lower(String::from("Привет, Tomáše!"));
		assert_eq!(page.text, "привет, tomáše!");
	}

	#[test]
	fn multiline() {
		let page = Page::create(String::from(
			"If you see Kay
tell him he may,
see you in tea,
tell him from me!",
		));
		assert_eq!(page.letters, 50);
		assert_eq!(page.length, 65);
		assert_eq!(
			page.text,
			"If you see Kay\ntell him he may,\nsee you in tea,\ntell him from me!"
		);
	}

	#[test]
	fn multiline_lower() {
		let page = Page::lower(String::from(
			"Скажи-ка дядя,
Ведь не даром,
Москва, спаленная пожаром,
Французу отдана?",
		));
		assert_eq!(
			page.text,
			"скажи-ка дядя,\nведь не даром,\nмосква, спаленная пожаром,\nфранцузу отдана?"
		);
	}

	#[test]
	fn multiline_windows() {
		let page = Page::create(String::from(
			"If you see Kay\ntell him he may,\nsee you in tea,\ntell him from me!",
		));
		assert_eq!(page.letters, 50);
		assert_eq!(page.length, 65);
		assert_eq!(
			page.text,
			"If you see Kay
tell him he may,
see you in tea,
tell him from me!"
		);
	}
}
