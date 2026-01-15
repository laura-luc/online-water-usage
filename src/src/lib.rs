use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, window};

struct AppState {
    total_liters: f64,
    unit_liters: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            total_liters: 0.0,
            unit_liters: true,
        }
    }
}

thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::new());
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}

#[wasm_bindgen]
pub fn add_usage() {
    let doc = document();

    let rate: f64 = doc
        .get_element_by_id("usageType")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .value()
        .parse()
        .unwrap();

    let usage_item = doc
        .get_element_by_id("usageType")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .selected_index();
    let usage_name = doc
        .get_element_by_id("usageType")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap()
        .item(usage_item.try_into().unwrap())
        .unwrap()
        .text_content()
        .unwrap();

    let mut time: f64 = doc
        .get_element_by_id("time")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value()
        .parse()
        .unwrap_or(0.0);

    if time <= 0.0 {
        return;
    }

    if usage_name == "LLM / AI" {
        time = time.round()
    }

    let liters = rate * time;

    STATE.with(|state: &RefCell<AppState>| {
        state.borrow_mut().total_liters += liters;
    });

    let li = doc.create_element("li").unwrap();
    li.set_class_name("list-group-item d-flex justify-content-between align-items-center");

    // Store liters on the element
    li.set_attribute("data-liters", &liters.to_string())
        .unwrap();

    if usage_name != "LLM / AI" {
        li.set_inner_html(&format!(
            "<span>{} - {:.1} h - {:.2} L</span>
             <button class='btn btn-sm btn-danger noPrint'>✕</button>",
            usage_name, time, liters
        ));
    } else if time == 1.0 {
        li.set_inner_html(&format!(
            "<span>{} -{:.0} query - {:.2} L</span>
             <button class='btn btn-sm btn-danger noPrint'>✕</button>",
            usage_name, time, liters
        ));
    } else {
        li.set_inner_html(&format!(
            "<span>{} - {:.0} queries - {:.2} L</span>
         <button class='btn btn-sm btn-danger noPrint'>✕</button>",
            usage_name, time, liters
        ));
    }

    // Attach delete handler
    let button = li.query_selector("button").unwrap().unwrap();

    let li_clone = li.clone();
    let closure = Closure::<dyn FnMut()>::new(move || {
        remove_usage(li_clone.clone());
    });

    button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget();

    doc.get_element_by_id("usageList")
        .unwrap()
        .append_child(&li)
        .unwrap();

    update_total();
}

#[wasm_bindgen]
pub fn toggle_unit() {
    STATE.with(|state: &RefCell<AppState>| {
        let mut s = state.borrow_mut();
        s.unit_liters = !s.unit_liters;
    });
    update_total();
}

#[wasm_bindgen]
pub fn toggle_dark() {
    let body = document().body().unwrap();
    body.class_list().toggle("dark").unwrap();
}

fn update_total() {
    STATE.with(|state: &RefCell<AppState>| {
        let s = state.borrow();
        let text = if s.unit_liters {
            format!("Total: {:.2} L", s.total_liters)
        } else {
            format!("Total: {:.2} gal", s.total_liters * 0.264172)
        };

        document()
            .get_element_by_id("total")
            .unwrap()
            .set_text_content(Some(&text));
    });
}

#[wasm_bindgen]
pub fn remove_usage(item: Element) {
    let liters: f64 = item.get_attribute("data-liters").unwrap().parse().unwrap();

    STATE.with(|state: &RefCell<AppState>| {
        state.borrow_mut().total_liters -= liters;
    });

    item.remove();
    update_total();
}
