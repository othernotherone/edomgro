use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, digit1, line_ending, multispace0, space1},
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

use super::{OrgError, OrgResult};
use super::ast::*;

pub fn parse_org_document(input: &str) -> OrgResult<OrgDocument> {
    match document(input) {
        Ok((_, doc)) => Ok(doc),
        Err(e) => Err(OrgError::ParseError(e.to_string())),
    }
}

fn document(input: &str) -> IResult<&str, OrgDocument> {
    let (input, nodes) = many0(org_node)(input)?;
    Ok((input, OrgDocument { title: None, nodes }))
}

fn org_node(input: &str) -> IResult<&str, OrgNode> {
    alt((
        map(heading, OrgNode::Heading),
        map(paragraph, OrgNode::Paragraph),
        map(list, OrgNode::List),
    ))(input)
}

fn heading(input: &str) -> IResult<&str, Heading> {
    let (input, (stars, _, title, tags)) = tuple((
        many1(char('*')),
        space1,
        take_until("\n"),
        opt(tags),
    ))(input)?;
    
    let (input, _) = line_ending(input)?;
    let (input, content) = many0(org_node)(input)?;
    
    Ok((input, Heading {
        level: stars.len() as u32,
        title: title.trim().to_string(),
        tags: tags.unwrap_or_default(),
        content,
    }))
}

fn tags(input: &str) -> IResult<&str, Vec<String>> {
    let (input, tags) = delimited(
        char(':'),
        many1(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
        char(':'),
    )(input)?;
    
    Ok((input, tags.iter().map(|&s| s.to_string()).collect()))
}

fn paragraph(input: &str) -> IResult<&str, String> {
    let (input, lines) = many1(tuple((
        take_until("\n"),
        line_ending,
    )))(input)?;
    
    Ok((input, lines.iter()
        .map(|(line, _)| *line)
        .collect::<Vec<&str>>()
        .join("\n")))
}

fn list(input: &str) -> IResult<&str, List> {
    alt((unordered_list, ordered_list))(input)
}

fn unordered_list(input: &str) -> IResult<&str, List> {
    let (input, items) = many1(unordered_list_item)(input)?;
    Ok((input, List {
        items,
        kind: ListKind::Unordered,
    }))
}

fn unordered_list_item(input: &str) -> IResult<&str, ListItem> {
    let (input, _) = tuple((
        multispace0,
        alt((tag("- "), tag("+ "), tag("* "))),
    ))(input)?;
    
    let (input, content) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;
    
    Ok((input, ListItem {
        content: content.trim().to_string(),
        sub_items: vec![],
    }))
}

fn ordered_list(input: &str) -> IResult<&str, List> {
    let (input, items) = many1(ordered_list_item)(input)?;
    Ok((input, List {
        items,
        kind: ListKind::Ordered,
    }))
}

fn ordered_list_item(input: &str) -> IResult<&str, ListItem> {
    let (input, _) = tuple((
        multispace0,
        digit1,
        char('.'),
        space1,
    ))(input)?;
    
    let (input, content) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;
    
    Ok((input, ListItem {
        content: content.trim().to_string(),
        sub_items: vec![],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let input = r#"* First Heading :tag1:tag2:
Some paragraph text
here.

- List item 1
- List item 2

** Sub heading
More text here.
"#;
        let result = parse_org_document(input);
        assert!(result.is_ok());
        
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1); // One top-level heading
        
        if let OrgNode::Heading(h) = &doc.nodes[0] {
            assert_eq!(h.level, 1);
            assert_eq!(h.title, "First Heading");
            assert_eq!(h.tags, vec!["tag1", "tag2"]);
        } else {
            panic!("Expected heading");
        }
    }
} 