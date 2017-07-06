// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.

#![deny(warnings)]

extern crate qui;
extern crate url;

use qui::{App, QuickView};
use std::path::Path;
use url::Url;

const QML_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"),
    "/examples/hello_world.qml");

fn main() {
    let app = App::new();
    const APP_NAME: &str = "Hello world";
    app.set_name(APP_NAME);
    assert_eq!(APP_NAME, app.name());
    app.set_style("Material");
    let view = QuickView::new(&app);
    view.set_source(&Url::from_file_path(Path::new(QML_FILE)).unwrap());
    view.show();
    let code = app.exec();
    println!("App exited with code {}.", code);
}
