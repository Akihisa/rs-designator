use super::token::{CLOSE_PAREN, OPEN_PAREN};
use core::cmp;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Designator {
    prefix: String,
    number: usize,
    suffix: Option<char>,
    has_paren: bool,
}

impl fmt::Display for Designator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_paren {
            f.write_char(OPEN_PAREN)?
        }

        f.write_str(&self.prefix)?;

        if self.number > 0 {
            write!(f, "{}", self.number)?;
        }

        if let Some(suffix) = self.suffix {
            f.write_char(suffix)?
        }

        if self.has_paren {
            f.write_char(CLOSE_PAREN)?
        }

        Ok(())
    }
}
impl From<&str> for Designator {
    fn from(s: &str) -> Self {
        let has_paren = s.starts_with(OPEN_PAREN) && s.ends_with(CLOSE_PAREN);

        let chars: Vec<char> = if has_paren {
            s.chars().skip(1).take(s.len() - 2).collect()
        } else {
            s.chars().collect()
        };

        // 空の参照名
        if chars.is_empty() {
            let mut designator = Designator::new();
            designator.has_paren = has_paren;
            return designator;
        }

        // 数字があるか
        let Some(num_start) = chars.iter().position(|c| c.is_ascii_digit()) else {
            // なければ単語の参照名
            return Designator::word(String::from_iter(chars), has_paren);
        };

        // 数字の終わり
        let num_end = num_start
            + chars
                .iter()
                .skip(num_start)
                .take_while(|c| c.is_ascii_digit())
                .count()
            - 1;

        // 接尾辞があるか
        // 接尾辞は1文字のアルファベット記号のみとし、それ以外は単語参照名
        // 接尾辞が複数文字の場合やアルファベットでない場合、計算が非常に煩雑になるため限定しておく
        let suffix_start = num_end + 1;
        // 接尾辞は複数文字か?
        if (suffix_start + 1) < chars.len() {
            return Designator::word(String::from_iter(chars), has_paren);
        }
        let suffix = chars.get(suffix_start).and_then(|c| Some(*c));
        // 接尾辞はアルファベットか？
        if suffix.is_some_and(|c| !c.is_ascii_alphabetic()) {
            return Designator::word(String::from_iter(chars), has_paren);
        }

        let number = String::from_iter(chars[num_start..=num_end].iter());

        Self {
            prefix: String::from_iter(chars.iter().take(num_start)),
            number: usize::from_str(&number).unwrap(),
            suffix,
            has_paren,
        }
    }
}

impl PartialEq for Designator {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl Eq for Designator {}

impl PartialOrd for Designator {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Designator {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.has_paren
            .cmp(&other.has_paren)
            .then(self.prefix.cmp(&other.prefix))
            .then(self.number.cmp(&other.number))
            .then(self.suffix.cmp(&other.suffix))
    }
}

impl Designator {
    fn new() -> Self {
        Self {
            prefix: String::new(),
            number: 0,
            suffix: None,
            has_paren: false,
        }
    }

    fn word(s: String, has_paren: bool) -> Self {
        Self {
            prefix: s,
            number: 0,
            suffix: None,
            has_paren,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.prefix.is_empty() && self.number == 0 && self.suffix.is_none()
    }

    pub fn is_word(&self) -> bool {
        !self.prefix.is_empty() && self.number == 0 && self.suffix.is_none()
    }
    // pub fn prefix(&self) -> &str {
    //     self.prefix.as_str()
    // }

    // pub fn number(&self) -> usize {
    //     self.number
    // }

    // pub fn suffix(&self) -> Option<char> {
    //     self.suffix
    // }

    pub fn has_paren(&self) -> bool {
        self.has_paren
    }

    pub fn without_parentheses(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            number: self.number,
            suffix: self.suffix,
            has_paren: false,
        }
    }

    pub fn to_omitted_string(&self, other: &Self) -> String {
        if self.is_empty() || other.is_empty() || self.is_word() || other.is_word() {
            return self.to_string();
        }

        let mut omitted = String::new();

        if self.prefix != other.prefix {
            omitted += self.prefix.as_str()
        }

        if self.number > 0 {
            omitted += self.number.to_string().as_str();
        }

        if let Some(suffix) = self.suffix {
            omitted.push(suffix);
        }

        omitted
    }

    pub fn next(&self) -> Option<Self> {
        if self.is_empty() || self.is_word() {
            return None;
        }

        if self.suffix.is_some() {
            let suffix = char::from(self.suffix.unwrap() as u8 + 1);
            Some(Self {
                prefix: self.prefix.clone(),
                number: self.number,
                suffix: Some(suffix),
                has_paren: self.has_paren,
            })
        } else {
            let number = self.number + 1;
            Some(Self {
                prefix: self.prefix.clone(),
                number,
                suffix: None,
                has_paren: self.has_paren,
            })
        }
    }

    pub fn complement(&mut self, other: &Designator) {
        // 自身が空の場合、他が空もしくは単語の場合は補完対象外
        if self.is_empty() || other.is_empty() || other.is_word() {
            return;
        }

        // R1a,b のような参照名の場合、b は接尾辞として扱う
        // パーサーからは b は接頭辞だけの参照名として扱われているはずなので、接尾辞から補完していく
        // (接尾辞と接頭辞を入れ替える)

        // 接尾辞の補完
        if other.suffix.is_some() {
            // 接頭辞が1文字かつアルファベットの場合だけ
            if self.is_word()
                && self.prefix.len() == 1
                && self
                    .prefix
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic())
            {
                self.suffix = self.prefix.pop(); // 1文字しかないので、pop 後 prefix は空
            }
            // 接頭辞と接尾辞が入れ替えられたら、数値も補完する
            if self.suffix.is_some() && self.number == 0 {
                self.number = other.number;
            }
        }

        // 数値は接尾辞があるときしか補完できない

        // 接頭辞の補完
        // 接頭辞がなければ、other と同じに
        if self.prefix.is_empty() {
            self.prefix = other.prefix.clone();
        }
    }

    pub fn difference(&self, other: &Designator) -> Option<isize> {
        // 括弧は無視して、差分をとる
        // prefix 違いは None
        if self.prefix != other.prefix {
            return None;
        }

        // suffix のあり/なしが異なる場合は None
        if self.suffix.is_some() != other.suffix.is_some() {
            return None;
        }

        if let Some(ss) = self.suffix {
            // prefix + number + suffix
            if self.number != other.number {
                // suffix がある場合 number は同じでないといけない
                return None;
            }
            // suffix ありなしが同じことの確認は済んでいるので、unwrap は失敗しない
            let os = other.suffix.unwrap();

            if ss.is_ascii_lowercase() != os.is_ascii_lowercase() {
                // 大文字小文字が異なる場合は None
                return None;
            }

            Some(ss as isize - os as isize)
        } else {
            // prefix + number
            Some(self.number as isize - other.number as isize)
        }
    }
}
