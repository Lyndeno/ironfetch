use std::fmt;
pub trait Fetchline {
    fn get_fetchline(&self) -> String;
}

pub fn gen_fetchline<T: fmt::Display>(v: Vec<Option<T>>) -> String {
    let mut index = 0;
    let mut s = String::new();
    for item in v {
       match item {
        Some(s2) => {
            if index > 0 { s.push_str(" ")};
            s.push_str(s2.to_string().as_str());
            index = index + 1;
        },
        None => {},
       };
    }
    s
}