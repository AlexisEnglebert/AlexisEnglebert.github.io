#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

#[macro_use] extern crate rocket;
extern crate maud;
extern crate ammonia;
extern crate pulldown_cmark;

use maud::{Markup, html, DOCTYPE, PreEscaped, Render};
use rocket::fs::FileServer;
use pulldown_cmark::{Parser, html};

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static_unsafe {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static {
	($path: expr) => (PreEscaped(include_static_unsafe!($path)))
}

macro_rules! include_md {
	($path: expr) => (Markdown(include_static_unsafe!($path)))
}

struct Markdown<T: AsRef<str>>(T);

impl <T: AsRef<str>> Render for Markdown<T> {
	fn render(&self) -> Markup {
		let mut unsafe_html = String::new();
		let parser = Parser::new(self.0.as_ref());

		html::push_html(&mut unsafe_html, parser);

		let safe = ammonia::clean(&unsafe_html);
		PreEscaped(safe)
	}
}

fn base(content: Markup) -> Markup {
	html! {
		(DOCTYPE)

		html lang="en" {
			head {
				meta charset="UTF-8"; // must be in the first 1024 bytes of the document
				meta name="description" content="Info on ICPC-related stuff at UCLouvain"; // can't be longer than 275 characters as per Google's 2017 limit on the SERP
				meta name="viewport" content="width=device-width,initial-scale=1";
				meta name="robots" content="index,follow";
				meta name="theme-color" content="#000000";

				link rel="icon" type="image/png" href="https://cdn.shopify.com/s/files/1/1061/1924/products/Nerd_with_Glasses_Emoji_2a8485bc-f136-4156-9af6-297d8522d8d1_large.png?v=1571606036";
				link rel="manifest" href="manifest.json";

				// Apple PWA stuff

				meta name="apple-mobile-web-app-capable" content="yes";
				meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
				meta name="apple-mobile-web-app-title" content="UCL ICPC";

				// TODO keywords, google-site-verification, apple-touch-startup-image

				title { "UCLouvain ICPC" }

				// link rel="stylesheet" type="text/css" href="/public/main.css";

				style {
					(include_static!("/main.css"))
				}
			}

			body {
				(content)
			}
		}
	}
}

#[get("/")]
fn index() -> Markup {
	base(html! {
		.balloon {
			// shaders

			script #vert-shader type="x-shader/x-vertex" { (include_static!("/balloon/vert.glsl")) }
			script #frag-shader type="x-shader/x-fragment" { (include_static!("/balloon/frag.glsl")) }

			// models

			script src="/public/balloon/model.js" defer {}

			// actual balloon

			canvas #balloon width="400px" height="500px";
			script src="/public/balloon/balloon.js" defer {}
		}
		
		.container {
			(include_md!("/md/index.md"))
		}
	})
}

// server stuff

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index])
		.mount("/public", FileServer::from(relative!("/public")))
}
