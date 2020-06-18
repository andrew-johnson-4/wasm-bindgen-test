#![feature(proc_macro_hygiene)]
use rdxl::{xtype,xrender,xhtml};
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

xtype!(<!MyAttr field:u64><!MyAttrChild field:u64/></MyAttr>);
xtype!(<!MyType attr:MyAttr/>);
xrender!(MyType, <ul>
  {{ for MyAttrChildren::MyAttrChild(c) in self.attr.children.iter() {{
    <li>{{ self.attr.field }}:{{ c.field }}</li>
  }} }}
</ul>);

#[wasm_bindgen_test]
fn complex_classes_as_attr(){
   assert_eq!(bs(xhtml!(<!MyType attr=<!MyAttr field=3>
       <!MyAttrChild field=2/>
       <!MyAttrChild field=1/>
     </MyAttr>/>)),
     "<ul> <li>3:2</li> <li>3:1</li> </ul>".to_string()
   );
}
