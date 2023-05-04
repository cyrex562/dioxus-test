use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    // println!("Hello, world!");
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
            div {
                form {
                    div {
                        label{

                        }
                        input{
                            name: "import_file_input",
                            r#type: "file",
                            onchange: move |evt| {
                                println!("onchange: {:?}", evt);

                            }
                        }
                    }
                    div {
                        button{
                            onclick: move |event| {
                        event.stop_propagation();

                        }
                        "import file"
                        }
                    }
                }
            }
        }
    })
}
