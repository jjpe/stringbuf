use std::ops;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringBuf(String);

impl StringBuf {
    pub fn new() -> Self { StringBuf(String::new()) }

    pub fn append<'s, S: Into<&'s str>>(self, string: S) -> Self {
        self + string.into()
    }
}

impl From<String> for StringBuf {
    fn from(string: String) -> StringBuf {  StringBuf(string)  }
}

impl From<StringBuf> for String {
    fn from(str_buf: StringBuf) -> String {  str_buf.0  }
}



impl<'s, S> ops::Add<S> for StringBuf where S: Into<&'s str> {
    type Output = StringBuf;

    fn add(mut self, string: S) -> StringBuf {
        self.0.push_str(string.into());
        self
    }
}


#[cfg(test)]
mod tests {
    use *;

    #[test]

    #[test]
    fn it_works() {
        let mut buf = StringBuf::new();
        buf = buf + "foo ";
        buf = buf + "bar ";
        buf = buf.append("baz ").append("qux!");

        let s = "BLAH!!!".to_string();
        buf = buf + &*s;
        println!("buf: {:?}", String::from(buf));
        assert!(false);
    }
}
