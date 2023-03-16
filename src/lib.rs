use std::borrow::Cow;
use std::fmt::Write;

pub trait StringExt {
    fn expand_tabs_default(&self) -> Cow<str> {
        self.expand_tabs(8)
    }

    fn expand_tabs(&self, tab_size: u16) -> Cow<str>;
}

impl<T> StringExt for T
where
    T: AsRef<str>,
{
    fn expand_tabs(&self, tab_size: u16) -> Cow<str> {
        let s = self.as_ref();
        let tab = '\t';
        if s.contains(tab) {
            let mut res = String::new();
            let mut last_pos = 0;

            while let Some(pos) = &s[last_pos..].find(tab) {
                res.push_str(&s[last_pos..*pos + last_pos]);

                let spaces_to_add = if tab_size != 0 {
                    tab_size - (*pos as u16 % tab_size)
                } else {
                    0
                };

                if spaces_to_add != 0 {
                    let _ = write!(res, "{:width$}", "", width = spaces_to_add as usize);
                }

                last_pos += *pos + 1;
            }

            res.push_str(&s[last_pos..]);

            Cow::from(res)
        } else {
            Cow::from(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringExt;

    #[test]
    fn test_default_tab_size_works() {
        assert_eq!("H       e", "H\te".expand_tabs_default());
    }

    #[test]
    fn test_tab_size_of_two_works() {
        assert_eq!("H e", "H\te".expand_tabs(2));
    }

    #[test]
    fn test_tab_size_of_four_works() {
        assert_eq!("H   e", "H\te".expand_tabs(4));
    }

    #[test]
    fn test_tab_size_of_one_works() {
        assert_eq!("H e", "H\te".expand_tabs(1));
    }

    #[test]
    fn test_tab_size_of_zero_works() {
        assert_eq!("He", "H\te".expand_tabs(0));
    }

    #[test]
    fn test_tab_size_of_three_works() {
        assert_eq!("H  e", "H\te".expand_tabs(3));
    }

    #[test]
    fn test_tab_size_of_nine_works() {
        assert_eq!("H        e", "H\te".expand_tabs(9));
    }

    #[test]
    fn test_tab_at_position_larger_than_tab_size_works() {
        assert_eq!("Hello World", "Hello\tWorld".expand_tabs(1));
        assert_eq!("HelloWorld", "Hello\tWorld".expand_tabs(0));
        assert_eq!("Hello   World", "Hello\tWorld".expand_tabs(4));
    }

    #[test]
    fn test_expand_multiple_tabs_works() {
        assert_eq!("H ello World", "H\tello\tWorld".expand_tabs(1));
    }
}
