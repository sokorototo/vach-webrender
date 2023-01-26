use std::io::Cursor;

use js_sys::Uint8Array;
use vach::prelude::Archive;
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, HtmlInputElement};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
struct FileData(Vec<u8>);

#[derive(Debug, Default)]
struct App(Option<Archive<Cursor<Vec<u8>>>>);

impl Component for App {
    type Message = Option<FileData>;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Default::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback_future(|ev: Event| {
            ev.target_dyn_into()
                .map(|elem: HtmlInputElement| {
                    elem.files().map(|files| async move {
                        match files.get(0) {
                            Some(f) => {
                                let blob: &Blob = f.as_ref();
                                let buf = JsFuture::from(blob.array_buffer()).await.unwrap_throw();

                                let data = FileData(Uint8Array::new(&buf).to_vec());

                                Some(data)
                            }
                            None => None,
                        }
                    })
                })
                .flatten()
                .expect_throw("Practically impossible errors, since element is an HtmlInputElement and must contain a File to change")
        });

        // Render this shiit!
        html! {
            <>
                <h1>{ if self.0.is_some() { "File Loaded Successfully" } else { "Pending File.." } }</h1>
                <input type="file" {onchange}/>
                {
                    self.0.as_ref().into_iter().flat_map(|ar| ar.entries()).map(|(id, entry)| {
                        html! {
                            <div>
                                <h2>{ id }</h2>
                                <p>{ "size=" }{ entry.offset }</p>
                                <p>{"content-version="}{ entry.content_version }</p>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Some(file) => {
                let base = Cursor::new(file.0);
                self.0 = Some(Archive::new(base).unwrap_throw());

                true
            }
            None => false,
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
