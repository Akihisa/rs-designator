use super::lexer::Lexer;
use super::token::*;
use crate::designator::Designator;
use std::slice::IterMut;
use std::vec::IntoIter;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        let mut tokens: Vec<TokenWithSymbol> = Vec::new();
        // 先頭のカンマ、ホワイトスペースを除く
        if let Some(tok) = self.skip_start() {
            tokens.push(tok);
        } else {
            return Vec::new();
        }
        // 以降を読み出す
        tokens.append(&mut self.read());

        // 最初の状態を文字列に変換して保存しておく
        // let src = tokens.iter().map(|tok| tok.symbol()).collect::<String>();
        // println!("{}", src);

        // インライン括弧の処理
        // 識別子の後の開き括弧から閉じ括弧までの識別子化
        replace_inline_parentheses(tokens.iter_mut());
        // 対応する括弧がない括弧の識別子化
        replace_invalid_parentheses(&mut tokens);
        // 括弧内の識別子への変換と括弧のホワイトスペース化
        // 括弧内の識別子をシンボルで判断できるようにした後、括弧をホワイトスペースに変換し、括弧を全てなくしまう
        // 削除するのではなくホワイトスペース化するのは区切りとして利用するため
        put_in_parentheses(tokens.iter_mut());
        // 範囲記号の左右以外のホワイトスペースをカンマへ変換
        replace_whitespace_to_comma(&mut tokens);
        // 不正な範囲記号の識別子化
        replace_invalid_range_symbols(&mut tokens);
        // トークン列の作り直し
        let mut tokens = rebuild(tokens.into_iter());
        // 範囲記号を前置記法に変換
        // 整列の後に実行しないと、識別子が結合される
        convert_prefix_notation(&mut tokens);

        // 最終結果
        // let rel = tokens.iter().map(|tok| tok.symbol()).collect::<String>();
        // println!("{}", rel);

        // Designator に変換する
        let designators: Vec<Designator> = convert_to_designators(tokens);

        designators.into_iter().map(|d| d.to_string()).collect()
    }

    fn skip_start(&mut self) -> Option<TokenWithSymbol> {
        while let Some(token) = self.lexer.token() {
            match token {
                t if t.is_whitespace() => (),
                t if t.is_comma() => (),
                _ => return Some(token),
            }
        }

        None
    }
    fn read(&mut self) -> Vec<TokenWithSymbol> {
        let mut tokens: Vec<TokenWithSymbol> = Vec::new();
        let mut separator: Option<TokenWithSymbol> = None;

        // カンマ、ホワイトスペースの連続は1つにまとめる
        while let Some(token) = self.lexer.token() {
            match token {
                t if t.is_whitespace() && separator.is_none() => separator = Some(t),
                t if t.is_whitespace() => (),
                t if t.is_comma() => separator = Some(t),
                t => {
                    if let Some(sep) = separator {
                        tokens.push(sep);
                        separator = None;
                    }
                    tokens.push(t)
                }
            }
        }

        tokens
    }
}

fn replace_inline_parentheses(tokens: IterMut<TokenWithSymbol>) {
    let mut depth: usize = 0;
    let mut last_symbol: Option<char> = None;

    for token in tokens {
        match &token {
            // 識別子のあとが括弧の時がインライン括弧モードの始まり
            t if t.is_open_paren()
                && last_symbol.is_some_and(|c| c.to_ascii_lowercase() == IDENTIFIER) =>
            {
                depth += 1
            }
            // カンマでネストされたインライン括弧モードを含め終わらせる(最高優先度)
            t if t.is_comma() => depth = 0,
            _ => (),
        }
        // インライン括弧内の処理
        if depth > 0 {
            // 識別子に変換
            token.convert_symbol_to_identifier();
            // 最後のシンボル記録
            last_symbol = Some(IDENTIFIER);
            // 閉じ括弧で括弧の数を減らす
            if token.is_close_paren() {
                depth -= 1;
            }
        } else {
            last_symbol = Some(token.symbol());
        }
    }
}

fn replace_invalid_parentheses(tokens: &mut Vec<TokenWithSymbol>) {
    let mut indices: Vec<usize> = Vec::new();

    for (i, token) in tokens.iter_mut().enumerate() {
        match token {
            tok if tok.is_open_paren() => indices.push(i),
            tok if tok.is_close_paren() && indices.pop().is_none() => {
                tok.convert_symbol_to_identifier();
            }
            _ => (),
        }
    }

    for i in indices {
        tokens[i].convert_symbol_to_identifier();
    }
}

fn put_in_parentheses(tokens: IterMut<TokenWithSymbol>) {
    let mut depth = 0;
    for token in tokens {
        match token {
            tok if tok.is_open_paren() => {
                depth += 1;
                tok.convert_symbol_to_whitespace();
            }
            tok if tok.is_close_paren() => {
                depth -= 1;
                tok.convert_symbol_to_whitespace();
            }
            tok if tok.is_identifier() && depth > 0 => tok.parenthesize(),
            _ => (),
        }
    }
}

fn replace_whitespace_to_comma(tokens: &mut Vec<TokenWithSymbol>) {
    // windows(3) を使った処理は、前後が必要になるため、先頭、末尾のホワイトスペースは置き換えられない
    // 先頭のホワイトスペースを変換
    tokens
        .iter_mut()
        .take_while(|tok| tok.is_whitespace())
        .for_each(|tok| tok.convert_symbol_to_comma());

    // 末尾のホワイトスペースを変換
    tokens
        .iter_mut()
        .rev()
        .take_while(|tok| tok.is_whitespace())
        .for_each(|tok| tok.convert_symbol_to_comma());

    // 前後が範囲記号でないとき
    let mut indices: Vec<usize> = Vec::new();

    for (i, w) in tokens.windows(3).enumerate() {
        // 空白の前後が範囲記号か？
        if w[0].is_range() {
            continue;
        }
        if w[2].is_range() {
            continue;
        }
        // 真ん中が空白か？
        if w[1].is_whitespace() {
            indices.push(i + 1);
        }
    }
    // カンマに置換
    for i in indices {
        tokens[i].convert_symbol_to_comma();
    }
}

fn replace_invalid_range_symbols(tokens: &mut Vec<TokenWithSymbol>) {
    let chunks = tokens.split_mut(|tok| tok.is_comma());

    for chunk in chunks {
        let contains_range_symbol_count = chunk.iter().filter(|tok| tok.is_range()).count();
        // 範囲記号がいくつあるか？
        if contains_range_symbol_count == 0 {
            continue;
        }
        // 範囲記号は1つのみ許容
        let mut is_valid = contains_range_symbol_count == 1;
        if is_valid {
            // 間にあるホワイトスペースを無視して、[識別子][範囲記号][識別子](i~i) となっているかチェック
            let mut iter = chunk.iter().filter(|tok| !tok.is_whitespace());
            is_valid &= iter.next().is_some_and(|tok| tok.is_identifier());
            is_valid &= iter.next().is_some_and(|tok| tok.is_range());
            is_valid &= iter.next().is_some_and(|tok| tok.is_identifier());
        }

        if !is_valid {
            for tok in chunk.iter_mut().filter(|tok| tok.is_range()) {
                tok.convert_symbol_to_identifier();
            }
        }
    }
}

fn rebuild(iter: IntoIter<TokenWithSymbol>) -> Vec<TokenWithSymbol> {
    let mut tokens: Vec<TokenWithSymbol> = Vec::new();

    for mut token in iter {
        let prev = tokens.last_mut();
        match prev {
            Some(p) if p.is_comma() && (token.is_whitespace() || token.is_comma()) => (),
            Some(p) if p.is_identifier() && token.is_identifier() => {
                // 先にチェックしているのでエラーにはならない
                let _ = p.merge_token(token.token());
            }
            _ => {
                // シンボルに合わせて変換したうえで追加する
                token.transform();
                tokens.push(token)
            }
        }
    }

    tokens
}

fn convert_prefix_notation(tokens: &mut Vec<TokenWithSymbol>) {
    let chunks = tokens
        .split_mut(|tok| tok.is_comma())
        .filter(|chunk| chunk.iter().any(|tok| tok.is_range()));

    for chunk in chunks {
        let pos = chunk.iter().position(|tok| tok.is_range()).unwrap();
        chunk.swap(0, pos);
    }
}

fn convert_to_designators(tokens: Vec<TokenWithSymbol>) -> Vec<Designator> {
    let mut designators: Vec<Designator> = Vec::new();

    for chunk in tokens.split(|tok| tok.is_comma()) {
        let mut iter = chunk.iter().peekable();
        let prev = designators.last();

        match iter.next() {
            Some(tok) if tok.is_identifier() => {
                let mut designator = Designator::from(tok.token().to_string().as_str());
                if let Some(prev) = prev {
                    designator.complement(prev);
                }
                designators.push(designator);
            }
            Some(tok) if tok.is_range() => {
                // 範囲として不正なときに戻すためのベクタ
                let mut elements: Vec<String> = Vec::new();
                // ホワイトスペースか？
                if iter.peek().is_some_and(|tok| tok.is_whitespace()) {
                    elements.push(String::new());
                    iter.next();
                }
                // 一つめの参照名
                let left = iter.next().unwrap().token().to_string();
                elements.insert(0, left.clone());
                // 範囲記号
                elements.push(tok.token().to_string());
                let mut left = Designator::from(left.as_str());
                if let Some(prev) = prev {
                    left.complement(prev);
                }
                // ホワイトスペースか？
                if iter.peek().is_some_and(|tok| tok.is_whitespace()) {
                    elements.push(String::new());
                    iter.next();
                }
                // 2つめの参照名
                let right = iter.next().unwrap().token().to_string();
                elements.push(right.clone());
                let mut right = Designator::from(right.as_str());
                right.complement(&left);

                // 2つの差分をとって、二つめの方が大きい場合
                if right.difference(&left).is_some_and(|diff| diff > 0) {
                    designators.push(left.clone());
                    while let Some(designator) = left.next() {
                        if right.difference(&designator).is_some_and(|diff| diff > 0) {
                            designators.push(designator.clone());
                        } else {
                            break;
                        }
                        left = designator;
                    }
                    designators.push(right);
                } else {
                    for elem in elements.split(|e| e.is_empty()) {
                        let s = String::from_iter(elem.iter().map(|s| s.as_str()));
                        designators.push(Designator::from(s.as_str()));
                    }
                }
            }
            _ => (),
        }
    }

    designators
}
