#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[wasm_bindgen_test]
fn breaking_for() {
   assert_eq!(
      &bs(xhtml!(<ul>
        <li>1</li>
        <li>{{ 2 }}</li>
        {{ for i in 3..5 {{
          <li>{{ i }}</li>
        }} }}
      </ul>)),
      "<ul> <li>1</li> <li>2</li> <li>3</li> <li>4</li> </ul>"
   );
}

#[wasm_bindgen_test]
fn breaking_while() {
   assert_eq!(
      &bs(xhtml!(<ul>
        {{ let mut i = 3; }}
        {{ while i>0 {{
          <li>{{ i }}</li>
          {{ i -= 1; }}
        }} }}
      </ul>)),
      "<ul> <li>3</li> <li>2</li> <li>1</li> </ul>"
   );
}

#[wasm_bindgen_test]
fn breaking_if() {
   assert_eq!(
      &bs(xhtml!(<ul>
        {{ let x = 5; }}
        {{ if x>4 {{
          <li>1</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else if x>4 {{
          <li>2</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else {{
          <li>3</li>
        }} }}
      </ul>)),
      "<ul> <li>1</li> <li>2</li> <li>3</li> </ul>"
   );
}
