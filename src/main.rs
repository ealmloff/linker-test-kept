use dioxus::prelude::*;

fn main() {
    libby::please_build_me();

    launch(|| rsx! {
        img {
            src: "/assets/66571940.jpeg",
            alt: "A cute cat",
            width: "100%",
            height: "100%",
        }
    })
}

