extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn define_obj(item: TokenStream) -> TokenStream {
    let obj = format!("{}", item);
    let mut obj = obj.split(',');
    let mut out = "trait Obj: std::fmt::Debug + std::fmt::Display {
        fn id(&self) -> ObjId;
    "
    .to_string();
    for obj in obj {
        let mut func_name = obj.split("Obj");
        // Clear Obj from que
        func_name.next();
        let temp = func_name.next().unwrap();
        let temp = temp.chars().fold(String::new(), |mut s, x| {
            s.push_str(&x.to_lowercase().to_string());
            s
        });
        let func_sigs = format!("fn as_{}(&self) -> Option<&{}> {{None}}\n fn as_{}_mut(&mut self) -> Option<&mut {}> {{None}}\n", temp, obj, temp, obj);
        out.push_str(&func_sigs);
    }
    out.push('}');

    out.parse().unwrap()
}

#[proc_macro_derive(ObjFn, attributes(id))]
pub fn derive_obj_fn(item: TokenStream) -> TokenStream {
    let obj = format!("{}", item);
    let mut type_name = obj.split(' ');
    // Clear 'struct'
    type_name.next();
    let type_name = type_name.next().unwrap();
    dbg!(type_name);
    let mut name = obj.split("Obj");
    // Clear Obj from que
    name.next();
    let name = name.next().unwrap();
    let mut name = name.split(' ');
    let name = name
        .next()
        .unwrap()
        .chars()
        .fold(String::new(), |mut s, x| {
            s.push_str(&x.to_lowercase().to_string());
            s
        });
    let mut field = obj.split('{');
    // Clear first match
    field.next();
    let rest = field.next().unwrap();
    let mut id = String::new();
    for i in rest.split(',') {
        if i.contains("#[id]") {
            id = i.to_string();
            break;
        } else {
            continue;
        }
    }
    let mut find_id = id.split(' ');
    // Clear first space
    find_id.next();
    // Clear id tag
    find_id.next();
    let id = find_id.next().unwrap();
    let out = format!(
        "impl Obj for {} {{
            fn id(&self) -> ObjId {{
            self.{}
        }}
        fn as_{}(&self) -> Option<&{}> {{
            Some(self)
        }}
        fn as_{}_mut(&mut self) -> Option<&mut {}> {{
            Some(self)
        }}
        }}",
        type_name, id, name, type_name, name, type_name
    );
    out.parse().unwrap()
}
