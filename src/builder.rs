use super::designator::Designator;
use super::token::{CLOSE_PAREN, COMMA, OPEN_PAREN, RANGE};

pub fn build(designators: Vec<String>) -> String {
    if designators.is_empty() {
        return String::new();
    }
    if designators.len() == 1 {
        return designators[0].clone();
    }

    let designators: (Vec<_>, Vec<_>) = designators
        .iter()
        .map(|s| Designator::from(s.as_ref()))
        .collect::<Vec<_>>()
        .into_iter()
        .partition(|des| des.has_paren());

    let mut designator = String::new();
    if !designators.1.is_empty() {
        designator.extend(build_inner(designators.1, false).chars());
    }

    if !designators.0.is_empty() {
        if !designator.is_empty() {
            designator += ",\n";
        }
        designator.extend(build_inner(designators.0, true).chars())
    }

    println!("{}", designator);

    String::new()
}

pub fn build_inner(designators: Vec<Designator>, has_paren: bool) -> String {
    let mut designators = if has_paren {
        // すべて括弧を外す
        designators
            .into_iter()
            .map(|des| des.without_parentheses())
            .collect::<Vec<_>>()
    } else {
        designators
    };
    // ソート
    designators.sort();

    let mut differences: Vec<(String, isize)> = vec![(designators[0].to_string(), -1)];

    differences.extend(
        designators
            .iter()
            .skip(1)
            .zip(designators.iter())
            .map(|(a, b)| (a.to_omitted_string(b), a.difference(b).unwrap_or(-1))),
    );

    let chunks = differences.chunk_by(|_, (_, b)| *b == 1);
    let mut designator = chunks.fold(String::new(), |mut acc, chunk| {
        let (mut v, _): (Vec<_>, Vec<_>) = chunk.iter().cloned().unzip();
        if !acc.is_empty() {
            acc.push(COMMA);
        }
        let mut sep = COMMA.to_string();
        if chunk.len() > 2 {
            v.drain(1..(v.len() - 1));
            sep = RANGE.to_string();
        }

        acc + v.join(sep.as_str()).as_str()
    });

    if has_paren {
        designator.insert(0, OPEN_PAREN);
        designator.push(CLOSE_PAREN);
    }

    designator
}
