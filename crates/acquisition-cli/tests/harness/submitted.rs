//! A form as a browser submits it (the HTML standard's form submission,
//! the part the pages use), read from a page's text: the one reader the
//! fill's pin (`src/web/fill.rs`, which includes this file) and the page
//! tests (`tests/web.rs`) share.

/// An HTML text's entities as the page writes them, read back.
pub fn unescaped(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// A tag's attributes, `name="value"` or a bare name, in order.
fn attributes(tag: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let mut rest = tag
        .trim_start_matches('<')
        .trim_start_matches(|c: char| c.is_ascii_alphanumeric());
    loop {
        rest = rest.trim_start();
        if rest.is_empty() || rest.starts_with('>') || rest.starts_with("/>") {
            return out;
        }
        let end = rest
            .find(|c: char| c == '=' || c == '>' || c.is_whitespace())
            .unwrap_or(rest.len());
        let name = rest[..end].to_string();
        rest = &rest[end..];
        if let Some(quoted) = rest.strip_prefix("=\"") {
            let close = quoted.find('"').expect("a closed attribute value");
            out.push((name, Some(unescaped(&quoted[..close]))));
            rest = &quoted[close + 1..];
        } else {
            out.push((name, None));
        }
    }
}

fn attribute<'a>(attrs: &'a [(String, Option<String>)], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.as_deref().unwrap_or(""))
}

/// Whether `text` is a valid floating-point number as HTML reads a
/// number box's value: a box holding any other sends it empty.
fn html_number(text: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let body = text.strip_prefix('-').unwrap_or(text);
    let (mantissa, exponent) = match body.find(['e', 'E']) {
        Some(at) => (&body[..at], Some(&body[at + 1..])),
        None => (body, None),
    };
    let mantissa_ok = match mantissa.split_once('.') {
        Some((whole, part)) => (whole.is_empty() || digits(whole)) && digits(part),
        None => digits(mantissa),
    };
    let exponent_ok = exponent.is_none_or(|e| digits(e.trim_start_matches(['+', '-'])));
    mantissa_ok && exponent_ok
}

/// A form's fields as a browser submits it, in the page's order: the
/// first form whose opening tag is `form`, an element another form owns
/// left out, a disabled control and an unticked box sending
/// nothing, a select its selected option or else its first, a number
/// box its value only where it reads as a number, a text box its value
/// with its line breaks stripped.
pub fn submitted(html: &str, form: &str) -> Vec<(String, String)> {
    let open = html
        .find(form)
        .unwrap_or_else(|| panic!("no form opens {form:?}"));
    let close = open + html[open..].find("</form>").expect("the form closes");
    let mut body = &html[open..close];
    body = &body[body.find('>').unwrap() + 1..];
    let mut out = Vec::new();
    while let Some(at) = body.find('<') {
        body = &body[at..];
        let end = body.find('>').expect("a closed tag") + 1;
        let tag = &body[..end];
        let attrs = attributes(tag);
        let owned = attribute(&attrs, "form").is_none();
        let on = attribute(&attrs, "disabled").is_none();
        let name = attribute(&attrs, "name").map(str::to_string);
        if tag.starts_with("<input") {
            let kind = attribute(&attrs, "type").unwrap_or("text");
            let value = attribute(&attrs, "value").unwrap_or("").to_string();
            if let (Some(name), true, true) = (name, owned, on) {
                match kind {
                    "checkbox" => {
                        if attribute(&attrs, "checked").is_some() {
                            out.push((name, value));
                        }
                    }
                    "number" => {
                        let sent = if html_number(&value) {
                            value
                        } else {
                            String::new()
                        };
                        out.push((name, sent));
                    }
                    "search" | "button" | "submit" => {}
                    // a hidden field sends its value as written
                    "hidden" => out.push((name, value)),
                    // a text box's value has its line breaks stripped
                    _ => out.push((name, value.replace(['\n', '\r'], ""))),
                }
            }
            body = &body[end..];
        } else if tag.starts_with("<select") {
            let shut = body.find("</select>").expect("a closed select");
            let options = &body[end..shut];
            let mut first = None;
            let mut chosen = None;
            for option in options.split("<option").skip(1) {
                let attrs = attributes(&format!(
                    "<option{}",
                    &option[..option.find('>').unwrap() + 1]
                ));
                let value = attribute(&attrs, "value").unwrap_or("").to_string();
                first.get_or_insert(value.clone());
                if attribute(&attrs, "selected").is_some() {
                    chosen = Some(value);
                }
            }
            if let (Some(name), true, true) = (name, owned, on)
                && let Some(value) = chosen.or(first)
            {
                out.push((name, value));
            }
            body = &body[shut..];
        } else if tag.starts_with("<textarea") {
            let shut = body.find("</textarea>").expect("a closed textarea");
            if let (Some(name), true, true) = (name, owned, on) {
                out.push((name, unescaped(&body[end..shut])));
            }
            body = &body[shut..];
        } else {
            body = &body[end..];
        }
    }
    out
}
