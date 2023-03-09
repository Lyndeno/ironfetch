use std::fmt;
pub trait Fetchline: fmt::Display {
    fn gen_fetchline<T: fmt::Display>(&self, v: Vec<Option<T>>) -> String {
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
}
