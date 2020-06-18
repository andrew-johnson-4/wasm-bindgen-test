#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

struct MyVal {}
impl MyVal {
   fn to_style(&self) -> String {
      "\"My:Val;\"".to_string()
   }
}

#[wasm_bindgen_test]
fn formatter1(){
   assert_eq!(
      xhtml!(<div style=[[ MyVal{} ]]>dave</div>),
      "<div style=\"My:Val;\">dave</div>".to_string()
   );
}

#[wasm_bindgen_test]
fn formatter2(){
   assert_eq!(
      xhtml!(<div an_attr={{ 2 }}>dave</div>),
      "<div an_attr=2>dave</div>".to_string()
   );
}
