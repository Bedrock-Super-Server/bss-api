// Reads the plugin ABI from the sources of bss-api, for the build scripts that generate the
// WebAssembly glue: the functions of `HostApi` and the layout of every event.
//
// Included with `include!` by the build script of bss-api (the imports of a plugin built to
// WebAssembly) and by that of bss-server (the host side of those imports), so both always follow
// the same sources.

/// A function of `HostApi`: its name, its parameters with their types, and its return type.
#[allow(dead_code)]
pub struct HostFn {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub ret: Option<String>,
}

/// An event: its stable number, the name of its data type and the fields after its header.
#[allow(dead_code)]
pub struct EventLayout {
    pub id: u32,
    pub ty: String,
    pub fields: Vec<(String, String)>,
}

fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Removes comments and attributes, which hold no part of a type.
fn code_lines(text: &str) -> String {
    text.lines()
        .filter(|line| {
            let line = line.trim_start();
            !line.starts_with("//") && !line.starts_with("#[")
        })
        .map(|line| line.split(" //").next().unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The body of `pub struct <name> {`, up to its closing brace.
fn struct_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("pub struct {name} {{");
    let start = source.find(&open)? + open.len();
    let mut depth = 1;
    for (i, c) in source[start..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[start..start + i]);
                }
            }
            _ => {}
        }
    }
    None
}

/// Splits `a, b<c, d>, e(f, g)` on the commas outside of brackets.
fn split_top(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    let mut previous = ' ';
    for c in text.chars() {
        match c {
            '(' | '<' | '[' => depth += 1,
            // The `>` of `->` closes nothing.
            '>' if previous == '-' => {}
            ')' | '>' | ']' => depth -= 1,
            _ => {}
        }
        previous = c;
        if c == ',' && depth == 0 {
            parts.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts.into_iter().filter(|part| !part.is_empty()).collect()
}

/// `name: type` pairs of a struct body or a parameter list.
fn fields_of(body: &str) -> Vec<(String, String)> {
    split_top(&squash(&code_lines(body)))
        .into_iter()
        .filter_map(|field| {
            let field = field.trim_start_matches("pub ").trim();
            let (name, ty) = field.split_once(':')?;
            Some((name.trim().to_string(), ty.trim().to_string()))
        })
        .collect()
}

/// The index of the `)` that closes the parenthesis opened just before `open`.
fn closing_paren(text: &str, open: usize) -> usize {
    let mut depth = 1;
    for (i, c) in text[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return open + i;
        }
    }
    panic!("unclosed parenthesis in {text}");
}

/// `Type = number, "name"` of an `event_data!` block: the type and the number.
#[allow(dead_code)]
fn event_entry(entry: &str) -> Option<(String, u32)> {
    let entry = squash(entry);
    let (ty, rest) = entry.split_once('=')?;
    let ty = ty.trim().to_string();
    let id = rest
        .split(',')
        .next()
        .and_then(|id| id.trim().parse().ok())
        .unwrap_or_else(|| panic!("number of {ty}"));
    Some((ty, id))
}

/// The functions of `HostApi`, in the order of the table.
#[allow(dead_code)]
pub fn host_functions(source: &str) -> Vec<HostFn> {
    let body = struct_body(source, "HostApi").expect("HostApi in host.rs");
    fields_of(body)
        .into_iter()
        .filter(|(name, _)| name != "header")
        .map(|(name, ty)| {
            let inner = ty
                .strip_prefix("Option<")
                .and_then(|ty| ty.strip_suffix('>'))
                .unwrap_or_else(|| panic!("HostApi::{name} is not an Option<fn>"));
            let open = inner
                .find("fn(")
                .unwrap_or_else(|| panic!("HostApi::{name} is not a function"))
                + 3;
            let close = closing_paren(inner, open);
            let ret = inner[close + 1..]
                .trim()
                .trim_end_matches(',')
                .trim()
                .strip_prefix("->")
                .map(|ret| ret.trim().to_string());
            HostFn {
                name,
                params: fields_of(&inner[open..close]),
                ret,
            }
        })
        .collect()
}

/// Every event declared with `event_data!` in `sources`, with the fields of its data type.
#[allow(dead_code)]
pub fn events(sources: &[String]) -> Vec<EventLayout> {
    let all = sources.join("\n");
    let mut events = Vec::new();
    for source in sources {
        let mut rest = source.as_str();
        while let Some(at) = rest.find("crate::event_data! {") {
            let block = &rest[at + "crate::event_data! {".len()..];
            let end = block.find('}').expect("end of event_data!");
            for (ty, id) in block[..end].split(';').filter_map(event_entry) {
                let body = struct_body(&all, &ty).unwrap_or_else(|| panic!("struct {ty}"));
                let fields = fields_of(body)
                    .into_iter()
                    .filter(|(name, _)| name != "header")
                    .collect();
                events.push(EventLayout { id, ty, fields });
            }
            rest = &block[end..];
        }
    }
    events.sort_by_key(|event| event.id);
    events
}

/// Size and alignment of a field type in a plugin built to `wasm32`.
#[allow(dead_code)]
pub fn wasm32_layout(ty: &str) -> (usize, usize) {
    match ty {
        "FfiStr" => (8, 4),
        "u64" | "i64" | "f64" => (8, 8),
        "u32" | "i32" | "f32" | "usize" => (4, 4),
        "u16" | "i16" => (2, 2),
        "u8" | "i8" | "bool" => (1, 1),
        other => panic!("no wasm32 layout for {other}"),
    }
}
