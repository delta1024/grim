extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(ObjFn, attributes(id))]
pub fn derive_obj_fn(item: TokenStream) -> TokenStream {
    let obj = format!("{}", item);
    let mut type_name = obj.split("struct");
    // Clear 'struct'
    type_name.next();
    let mut type_name = type_name.next().unwrap().split('{');
    let type_name = type_name.next().unwrap().trim();
    let name = type_name.chars().fold(String::new(), |mut s, x| {
        s.push_str(&x.to_lowercase().to_string());
        s
    });
    // Clear first match
    let out = format!(
        "impl Obj for {} {{
            fn obj_id(&self) -> ObjId {{
            self.id
        }}
        fn as_{}(&self) -> Option<&{}> {{
            Some(self)
        }}
        fn as_{}_mut(&mut self) -> Option<&mut {}> {{
            Some(self)
        }}
        }}",
        type_name, name, type_name, name, type_name
    );
    out.parse().unwrap()
}
