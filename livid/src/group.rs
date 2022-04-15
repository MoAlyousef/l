use crate::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};
use std::cell::RefCell;
// use wasm_bindgen::JsValue;

thread_local! {
    pub static PARENTS: RefCell<Vec<Widget>> = RefCell::from(vec![]);
}

pub struct Group {
    inner: Widget,
}

impl WidgetBase for Group {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
            p.borrow_mut().push(inner.clone());
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Group {}

impl GroupExt for Group {}

pub struct Column {
    inner: Widget,
}

impl WidgetBase for Column {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_class_name("flex-container");
        inner.set_style(Style::Display, "flex");
        inner.set_style(Style::FlexDirection, "column");
        inner.set_style(Style::AlignContent, "space-between");
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
            p.borrow_mut().push(inner.clone());
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Column {}

impl GroupExt for Column {}

pub struct Row {
    inner: Widget,
}

impl WidgetBase for Row {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_class_name("flex-container");
        inner.set_style(Style::Display, "flex");
        inner.set_style(Style::FlexDirection, "row");
        inner.set_style(Style::AlignContent, "space-between");
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
            p.borrow_mut().push(inner.clone());
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Row {}

impl GroupExt for Row {}