#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

struct MyMarkup {
   a:u64
}
impl MyMarkup {
   fn to_markup(&self) -> String {
      format!("{{a:{}}}", self.a)
   }
}

#[wasm_bindgen_test]
fn context_test1(){
   let x = MyMarkup { a:22 };
   assert_eq!(
      xhtml!([[ x ]]),
      "{a:22}".to_string()
   );
}
