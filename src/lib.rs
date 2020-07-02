use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture};

use web_sys::{console,Element,Event,MouseEvent,HtmlDocument,HtmlInputElement,HtmlTextAreaElement,IdbDatabase,IdbOpenDbOptions,IdbRequest,IdbRequestReadyState,IdbOpenDbRequest,IdbTransactionMode};

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn document() -> web_sys::Document {
    window()
        .document()
        .unwrap()
}

fn body() -> web_sys::HtmlElement {
    document()
        .body()
        .unwrap()
}

fn get_input(id: &str) -> Option<HtmlInputElement> {
    match document().get_element_by_id(id) {
        Some(v) => {
            match v.dyn_into::<HtmlInputElement>() {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
        None => None,
    }
}

fn get_textbox(id: &str) -> Option<HtmlTextAreaElement> {
    match document().get_element_by_id(id) {
        Some(v) => {
            match v.dyn_into::<HtmlTextAreaElement>() {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
        None => None,
    }
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {

    let db = window().indexed_db().unwrap().unwrap();
    let db_request = db.open_with_u32("test_db", 1).unwrap();
    
    let db_rc: Rc<RefCell<Option<IdbDatabase>>> = Rc::new(RefCell::new(None));
    let db_clone = db_rc.clone();
    
    let request_rc: Rc<RefCell<Option<IdbRequest>>> = Rc::new(RefCell::new(None));
    
    let db_request_callback = Closure::wrap(Box::new(move |event: Event| {
        let db_req = event.target().unwrap().dyn_into::<IdbOpenDbRequest>().unwrap();
        let db_result = db_req.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        db_clone.borrow_mut().replace(db_result);
        
        let db_trans = db_clone.borrow_mut().as_ref().unwrap().transaction_with_str_and_mode("table_1", IdbTransactionMode::Readwrite);
        match db_trans {
            Ok(db_trans) => {
                //~ console::log_1(&db_trans);
                let ob_store = db_trans.object_store("table_1");
                match ob_store {
                    Ok(ob_store) => {
                        //~ console::log_1(&v);
                        ob_store.add_with_key(&JsValue::from_str("bar"), &JsValue::from_str("b-00")).unwrap();
                        ob_store.add_with_key(&JsValue::from_str("bar"), &JsValue::from_str("b-01")).unwrap();
                        /*
                        let input = get_input("text_input");
                        match input {
                            Some(v) => {
                                let button_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
                                    let textbox = get_textbox("textbox");
                                    match textbox {
                                        Some(v) => {
                                            let res = ob_store.put_with_key(&JsValue::from_str(&v.value()), &JsValue::from_str("foo"));
                                            match res {
                                                Ok(_) => console::log_1(&"modified data".into()),
                                                Err(e) => console::log_1(&e),
                                            }
                                        }
                                        None => console::log_1(&"failed to find textbox".into()),
                                    }
                                }) as Box<dyn FnMut(_)>);
                                v.add_event_listener_with_callback("click", button_callback.as_ref().unchecked_ref()).unwrap();
                                button_callback.forget();
                            }
                            None => console::log_1(&"failed to find input".into()),
                        }
                        */
                    }
                    Err(e) => console::log_1(&e),
                }
                
            }
            Err(e) => console::log_1(&e),
        }
        
    }) as Box<dyn FnMut(_)>);    
    db_request.set_onsuccess(Some(&db_request_callback.as_ref().unchecked_ref()));
    db_request_callback.forget();
    
    let update_callback = Closure::wrap(Box::new(move |event: Event| {
        console::log_1(&"update callback".into());
        
        let db_req = event.target().unwrap().dyn_into::<IdbOpenDbRequest>().unwrap();
        let db_result = db_req.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        
        let ob_store = db_result.create_object_store("table_1");
        match ob_store {
            Ok(v) => {
                console::log_1(&v);
                //~ v.create_index_with_str("foo", "").unwrap();
                v.create_index_with_str("b-00", "").unwrap();
                v.create_index_with_str("b-01", "").unwrap();
                
            }
            Err(e) => console::log_1(&e),
        }
        
    }) as Box<dyn FnMut(_)>);
    db_request.set_onupgradeneeded(Some(&update_callback.as_ref().unchecked_ref()));
    update_callback.forget();
    
    let db_clone = db_rc.clone();
    let input = get_input("text_input");
    match input {
        Some(input) => {
            let button_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
                let textbox = get_textbox("textbox");
                match textbox {
                    Some(textbox) => {
                        let db_trans = db_clone.borrow_mut().as_ref().unwrap().transaction_with_str_and_mode("table_1", IdbTransactionMode::Readwrite);
                        match db_trans {
                            Ok(db_trans) => {
                                //~ console::log_1(&db_trans);
                                let ob_store = db_trans.object_store("table_1");
                                match ob_store {
                                    Ok(ob_store) => {
                                        let res = ob_store.put_with_key(&JsValue::from_str(&textbox.value()), &JsValue::from_str("b-00"));
                                        match res {
                                            Ok(_) => console::log_1(&"modified data".into()),
                                            Err(e) => console::log_1(&e),
                                        }
                                    }
                                    Err(e) => console::log_1(&e),
                                }
                            }
                            Err(e) => console::log_1(&e),
                        }
                        
                        /*
                        let db = window().indexed_db().unwrap().unwrap();
                        let db_request = db.open_with_u32("test_db", 1).unwrap();
                        let db_request_callback = Closure::wrap(Box::new(move |event: Event| {
                            let db_req = event.target().unwrap().dyn_into::<IdbOpenDbRequest>().unwrap();
                            let db_result = db_req.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
                            
                            let db_trans = db_result.transaction_with_str_and_mode("table_1", IdbTransactionMode::Readwrite);
                            match db_trans {
                                Ok(db_trans) => {
                                    console::log_1(&db_trans);
                                    let ob_store = db_trans.object_store("table_1");
                                    match ob_store {
                                        Ok(ob_store) => {
                                            let res = ob_store.put_with_key(&JsValue::from_str(&textbox.value()), &JsValue::from_str("foo"));
                                            match res {
                                                Ok(_) => console::log_1(&"modified data".into()),
                                                Err(e) => console::log_1(&e),
                                            }
                                        }
                                        Err(e) => console::log_1(&e),
                                    }
                                }
                                Err(e) => console::log_1(&e),
                            }
                        }) as Box<dyn FnMut(_)>);    
                        db_request.set_onsuccess(Some(&db_request_callback.as_ref().unchecked_ref()));
                        db_request_callback.forget();
    
                        //~ let res = ob_store.put_with_key(&JsValue::from_str(&textbox.value()), &JsValue::from_str("foo"));
                        //~ match res {
                            //~ Ok(_) => console::log_1(&"modified data".into()),
                            //~ Err(e) => console::log_1(&e),
                            */
                    }
                    None => console::log_1(&"failed to find textbox".into()),
                }
            }) as Box<dyn FnMut(_)>);
            input.add_event_listener_with_callback("click", button_callback.as_ref().unchecked_ref()).unwrap();
            button_callback.forget();
        }
        None => console::log_1(&"failed to find input".into()),
    }
    
    let db_clone = db_rc.clone();
    let request_clone = request_rc.clone();
    let click_callback = Closure::wrap(Box::new(move |event: Event| {
        //~ console::log_1(&event);
        match event.target() {
            Some(target) => {
                //~ console::log_1(&target);
                //~ console::log_1(&target.id);
                match target.dyn_into::<HtmlInputElement>() {
                    Ok(element) => {
                        //~ console::log_1(&element);
                        /*
                        match element.id().as_str() {
                            "b-00" => {
                                console::log_1(&"button 0 pressed".into());
                                let db_trans = db_clone.borrow_mut().as_ref().unwrap().transaction_with_str_and_mode("table_1", IdbTransactionMode::Read);
                                match db_trans {
                                    Ok(db_trans) => {
                                        //~ console::log_1(&db_trans);
                                        let ob_store = db_trans.object_store("table_1");
                                        match ob_store {
                                            Ok(ob_store) => {
                                                let res = ob_store.put_with_key(&JsValue::from_str(&textbox.value()), &JsValue::from_str("foo"));
                                                match res {
                                                    Ok(_) => console::log_1(&"modified data".into()),
                                                    Err(e) => console::log_1(&e),
                                                }
                                            }
                                            Err(e) => console::log_1(&e),
                                        }
                                    }
                                    Err(e) => console::log_1(&e),
                                }
                            }
                            _ => (),
                        }
                        */
                        
                        if element.class_name() == "paste" {
                            //~ console::log_1(&"button 0 pressed".into());
                            let db_trans = db_clone.borrow_mut().as_ref().unwrap().transaction_with_str_and_mode("table_1", IdbTransactionMode::Readonly);
                            match db_trans {
                                Ok(db_trans) => {
                                    //~ console::log_1(&db_trans);
                                    let ob_store = db_trans.object_store("table_1");
                                    match ob_store {
                                        Ok(ob_store) => {
                                            let res = ob_store.get(&JsValue::from_str(&element.id()));
                                            
                                            match res {
                                                Ok(res) => {
                                                    //~ console::log_1(&res);
                                                    request_clone.borrow_mut().replace(res);
                                                    
                                                }
                                                Err(e) => console::log_1(&e),
                                            }
                                        }
                                        Err(e) => console::log_1(&e),
                                    }
                                }
                                Err(e) => console::log_1(&e),
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
            None => {
                console::log_1(&"failed to get click target".into());
            }
        }
    }) as Box<dyn FnMut(_)>);
    window().add_event_listener_with_callback("mousedown", click_callback.as_ref().unchecked_ref())?;
    click_callback.forget();
    
    let request_clone = request_rc.clone();
    let request_timer_callback = Closure::wrap(Box::new(move || {
        let mut ready = false;
        match request_clone.borrow().as_ref() {
            Some(v) => {
                //~ console::log_1(&v);
                if v.ready_state() == IdbRequestReadyState::Done {
                    ready = true;
                }
            }
            None => {
                //~ console::error_1(&"request not available".into());
            }
        }
        
        if ready == true {
            let val = request_clone.borrow_mut().take();
            match val {
                Some(val) => {
                    //~ console::log_1(&val);
                    //~ request_clone.replace(None);
                    match val.result() {
                        Ok(result) => {
                            //~ console::log_1(&result);
                            match get_textbox("textbox") {
								Some(textbox) => {
									textbox.set_value(&result.as_string().unwrap());
									//~ textbox.select();
									let html = document().dyn_into::<HtmlDocument>().unwrap();
									textbox.select();
									//~ html.set_design_mode("on");
									//~ console::log_1(&html);
									html.exec_command("copy");
								}
								None => (),
							}
                        }
                        Err(_) => (),
                    }
                }
                None => (),
            }
        }
    }) as Box<dyn FnMut()>);
    window().set_interval_with_callback_and_timeout_and_arguments_0(request_timer_callback.as_ref().unchecked_ref(), 250).unwrap();
    request_timer_callback.forget();
    Ok(())
}
