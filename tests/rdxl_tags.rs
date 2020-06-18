#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn tag1() {
   assert_eq!(
      xhtml!("<div></div>"),
      "<div></div>".to_string()
   );
}

#[wasm_bindgen_test]
fn tag2() {
   assert_eq!(
      xhtml!(<div>dave</div><div>david</div>),
      "<div>dave</div><div>david</div>".to_string()
   );
}

#[wasm_bindgen_test]
fn tag3() {
   assert_eq!(
      xhtml!(<a href="there">this that</a><br/><p attr="something.f()">that this</p>),
      "<a href=\"there\">this that</a><br/><p attr=\"something.f()\">that this</p>".to_string()
   );
}

#[wasm_bindgen_test]
fn tag4() {
   assert_eq!(
      xhtml!(<input type="text"/>),
      "<input type=\"text\"/>".to_string()
   );
}

