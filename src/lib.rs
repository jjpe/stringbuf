use std::fmt;
use std::ops;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringBuf(String);

impl StringBuf {
    pub fn new() -> Self { StringBuf(String::new()) }

    pub fn append<'s, S: Into<&'s str>>(self, string: S) -> Self {
        self + string.into()
    }
}


impl fmt::Display for StringBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl<'s> From<&'s str> for StringBuf {
    fn from(string: &'s str) -> StringBuf {  StringBuf(string.to_string())  }
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

impl<'s, S> ops::AddAssign<S> for StringBuf where S: Into<&'s str> {
    fn add_assign(&mut self, string: S) {
        self.0.push_str(string.into());
    }
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn instantiation() {
        let sb = StringBuf::new();
        assert_eq!("", String::from(sb))
    }

    #[test]
    fn append() {
        let sb = StringBuf::new().append("foo").append("bar!");
        assert_eq!("foobar!", String::from(sb));
    }


    #[test]
    fn add() {
        let sb = StringBuf::new();
        let s = String::from("baz!");
        let sb = sb + "foo" + "bar!" + &*s;
        assert_eq!("foobar!baz!", String::from(sb));
    }

    #[test]
    fn add_assign() {
        let mut sb = StringBuf::new();
        sb += "foo";
        sb += "bar!";
        assert_eq!("foobar!", String::from(sb));
    }
}
