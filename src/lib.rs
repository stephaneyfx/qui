// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.

//! GUI library using QML

#![deny(warnings)]
#![deny(missing_docs)]

extern crate clue_sys as clue;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate qlue_sys as qlue;
extern crate url;

mod app;
mod quick_view;

pub use app::{App, AppRef};
pub use quick_view::QuickView;
