#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn misc1(){
   let mut x = 5;
   assert_eq!(xhtml!(<div>
      {{ x }},
      {{ x = 3; }}
      {{ x }},
      {{ x = 7; }}
      {{ x }},
      {{ let mut y = 2; }}
      {{ y }},
      {{ y = 1; }}
      {{ y }}
   </div>),
   "<div> 5, 3, 7, 2, 1 </div>".to_string());
}

#[wasm_bindgen_test]
fn misc2() {
   assert_eq!(
     xhtml!(<div "x-y"=5>6</div>),
     "<div x-y=5>6</div>".to_string()
   );
}
