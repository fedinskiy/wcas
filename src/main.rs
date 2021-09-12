use warp::Filter;

#[tokio::main]
async fn main() {
	let template=r##"
		<body><!DOCTYPE html>
<html>
<head>
    <title>Знакосчиталка</title>
</head>
<body>

<p>В тексте ${letters} символов, без пробелов</p>
<p>В тексте ${length} символов, с пробелами</p>

<div style="width: 80vw">
    <textarea style="width: 80vw; height: 30vh" id="content" name="input"  form="input">${was}</textarea>
    <br>
    <br>
    <form style="float: right;" action="/counter" id="clean" method="get">
        <input type="submit" value="Очистить">
    </form>
    <form style="float: left;" action="/counter" id="input" method="post">
        <input type="submit" value="Рассчитать">
    </form>
</div>
</body>"##;

	let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

	let html = warp::path!("counter").map(move || warp::reply::html(template) );

	let routes = warp::get().and(hello).or(html);
	
	
	warp::serve(routes)
		.run(([127,0,0,1],3030))
	.await;
}
