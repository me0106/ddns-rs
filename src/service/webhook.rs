use crate::model::{DnsConfig, DnsState, Webhook};
use reqwest::{
    Body, Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::{borrow::Cow, collections::HashMap};
use tracing::info;

pub async fn notify(config: &DnsConfig, webhook: &Webhook) -> anyhow::Result<String> {
    let parser::Request {
        method,
        uri,
        headers,
        body,
    } = parser::parse(&webhook.value)
        .map_err(|e| anyhow::anyhow!(format!("failed to parse request: \n{:#}", e)))?;
    let mut req_headers = HeaderMap::new();
    for (n, v) in headers {
        req_headers.insert(HeaderName::from_bytes(n)?, HeaderValue::from_bytes(v)?);
    }
    let variables = construct_variables(config);
    let uri = replace_variables(str::from_utf8(uri)?, &variables);
    let body = replace_variables(str::from_utf8(body)?, &variables);
    let builder = Client::new()
        .request(method, uri.to_string())
        .headers(req_headers)
        .body(Body::from(body.to_string()));
    let response = builder.send().await?.text().await?;
    info!("webhook response: {}", response);
    Ok(response)
}

fn replace_variables<'a>(template: &'a str, variables: &HashMap<&str, String>) -> Cow<'a, str> {
    let mut template = Cow::Borrowed(template);
    for (key, value) in variables {
        let key = format!("#{{{key}}}");
        let Some(begin) = template.find(&key) else {
            continue;
        };
        template
            .to_mut()
            .replace_range(begin..begin + key.len(), value);
    }
    template
}

fn construct_variables(config: &DnsConfig) -> HashMap<&'static str, String> {
    let mut variables = HashMap::new();
    variables.insert("domain", config.domain.to_string());
    if let Some(ipv4) = &config.ipv4
        && let Some(state) = &ipv4.state
    {
        match state {
            DnsState::Succeed { addr, .. } => {
                variables.insert("ipv4.state", "succeed".to_string());
                variables.insert("ipv4.addr", addr.to_string());
            }
            DnsState::Failed { message, .. } => {
                variables.insert("ipv4.state", "failed".to_owned());
                variables.insert("ipv4.message", message.to_owned());
            }
        }
    }
    let mut variables = HashMap::new();
    if let Some(ipv6) = &config.ipv6
        && let Some(state) = &ipv6.state
    {
        match state {
            DnsState::Succeed { addr, .. } => {
                variables.insert("ipv6.state", "succeed".to_string());
                variables.insert("ipv6.addr", addr.to_string());
            }
            DnsState::Failed { message, .. } => {
                variables.insert("ipv6.state", "failed".to_owned());
                variables.insert("ipv6.message", message.to_owned());
            }
        }
    }
    variables
}
mod parser {
    use reqwest::Method;
    use winnow::{
        ModalResult, Parser,
        ascii::{line_ending, multispace0, space0, space1, till_line_ending},
        combinator::{
            alt, cut_err, delimited, eof, opt, preceded, repeat, separated_pair, terminated,
        },
        error::{ContextError, ParseError, StrContext::*},
        token::{rest, take_while},
    };

    #[derive(Debug, PartialEq, Eq)]
    pub struct Request<'a> {
        pub method: Method,
        pub uri: &'a [u8],
        pub headers: Vec<(&'a [u8], &'a [u8])>,
        pub body: &'a [u8],
    }
    pub(crate) type Stream<'a> = &'a [u8];

    pub fn parse(input: &str) -> winnow::Result<Request, ParseError<Stream, ContextError>> {
        let mut input = input.as_bytes();
        request.parse(&mut input)
    }

    fn request<'a>(input: &mut Stream<'a>) -> ModalResult<Request<'a>> {
        let trim: for<'b> fn(&'b [u8]) -> &'b [u8] = |value| value.trim_ascii();
        let method = cut_err(preceded(
            multispace0,
            alt(("GET".value(Method::GET), "POST".value(Method::POST))),
        ))
        .context(Expected("GET or POST".into()))
        .parse_next(input)?;
        let uri = preceded(
            space1,
            terminated(till_line_ending, alt((line_ending, eof))),
        )
        .map(trim)
        .parse_next(input)?;
        let headers = repeat(
            0..,
            delimited(
                space0,
                separated_pair(
                    take_while(1.., token),
                    cut_err(delimited(space0, ':', space0)).context(Expected(":".into())),
                    till_line_ending.map(trim),
                ),
                opt(line_ending),
            ),
        )
        .parse_next(input)?;
        //clear space
        space0.parse_next(input)?;
        // newline for body
        let line: Option<Vec<&[u8]>> =
            opt(repeat(1.., delimited(space0, line_ending, space0))).parse_next(input)?;
        let body = match line {
            None => eof.parse_next(input)?,
            Some(_) => rest.parse_next(input)?,
        };
        Ok(Request {
            method,
            uri,
            headers,
            body,
        })
    }
    fn token(c: u8) -> bool {
        c.is_ascii_alphanumeric()
            || matches!(
                c,
                b'!' | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
            )
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn normal() {
            let mut input = r"
            POST https://example.com
            Content-Type: application/json

            {}"
            .as_bytes();
            let result = request.parse(&mut input);
            assert_eq!(
                result.unwrap(),
                Request {
                    method: Method::POST,
                    uri: "https://example.com".as_bytes(),
                    headers: vec![("Content-Type".as_bytes(), "application/json".as_bytes())],
                    body: "{}".as_bytes()
                }
            )
        }

        #[test]
        fn header_value_absent() {
            let mut input = r"
            POST https://example.com
            Content-Type: application/json
            dsad"
                .as_bytes();
            let result = request.parse(&mut input);
            assert_eq!(
                result.unwrap_err().to_string(),
                "parse error at line 4, column 17
  |
4 |             dsad
  |                 ^
expected `:`"
            );
        }

        #[test]
        fn nobody() {
            let mut input = r"
            POST https://example.com
            Content-Type: application/json   "
                .as_bytes();
            let result = request.parse(&mut input);
            assert_eq!(
                result.unwrap(),
                Request {
                    method: Method::POST,
                    uri: "https://example.com".as_bytes(),
                    headers: vec![("Content-Type".as_bytes(), "application/json".as_bytes())],
                    body: &[]
                }
            )
        }
        #[test]
        fn only_request_line() {
            let mut input = r"
             POST https://example.com
              "
            .as_bytes();
            let result = request.parse(&mut input);
            assert_eq!(
                result.unwrap(),
                Request {
                    method: Method::POST,
                    uri: "https://example.com".as_bytes(),
                    headers: vec![],
                    body: &[]
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extract() {
        let a = "GET https://google.com/?ipv4Addr=#{ipv4.addr}&ipv6Addr=#{ipv6.addr}";

        let mut map = HashMap::new();
        map.insert("ipv4.addr", "127.0.0.1".to_owned());
        map.insert("ipv6.addr", "::1".to_owned());
        let value = replace_variables(a, &map);
        assert_eq!(
            value,
            "GET https://google.com/?ipv4Addr=127.0.0.1&ipv6Addr=::1"
        )
    }
    #[test]
    fn test_raw() {
        let a = "GET https://google.com/?ipv4Addr=#{ipv4.addr&ipv6Addr=#{ipv6.addr}";

        let mut map = HashMap::new();
        map.insert("ipv4.addr", "127.0.0.1".to_owned());
        map.insert("ipv6.addr", "::1".to_owned());
        let value = replace_variables(a, &map);
        assert_eq!(
            value,
            "GET https://google.com/?ipv4Addr=#{ipv4.addr&ipv6Addr=::1"
        )
    }
}
