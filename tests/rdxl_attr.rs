#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn attribute_no_spaces(){
   assert_eq!(
      xhtml!(<div style="color:#FFFFFF; background-color:#000000;">dave</div>),
      "<div style=\"color:#FFFFFF; background-color:#000000;\">dave</div>".to_string()
   );
}
