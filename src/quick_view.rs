// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.

//! QML view module

use App;
use clue::convert::to_string_view;
use qlue;
use std::marker::PhantomData;
use url::Url;

/// View to load a QML scene.
///
/// Wraps `QQuickView`.
#[derive(Debug)]
pub struct QuickView<'a> {
    app: PhantomData<&'a App>,
    im: qlue::QlueQuickView,
}

impl<'a> QuickView<'a> {
    /// Creates a `QuickView` instance.
    pub fn new(_: &App) -> QuickView {
        unsafe {
            QuickView {
                app: PhantomData,
                im: qlue::qlueQuickViewNew(),
            }
        }
    }

    /// Loads QML file into the view.
    pub fn set_source(&self, url: &Url) {
        unsafe {
            qlue::qlueQuickViewSetSource(self.im, to_string_view(url.as_str()));
        }
    }

    /// Makes the view visible.
    pub fn show(&self) {
        unsafe {qlue::qlueQuickViewShow(self.im);}
    }
}

impl<'a> Drop for QuickView<'a> {
    fn drop(&mut self) {
        unsafe {qlue::qlueQuickViewDelete(self.im);}
    }
}
