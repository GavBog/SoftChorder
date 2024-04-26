use std::collections::HashMap;

pub fn sort_string(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    s.clear();
    for c in chars {
        s.push(c);
    }
}

pub async fn chord_to_string(actions: Vec<u16>, keymaps: &HashMap<u16, Option<String>>) -> String {
    let mut result = String::new();
    for action in actions {
        if let Some(action) = keymaps.get(&action) {
            if let Some(action) = action {
                result.push_str(action);
            }
        }
    }
    result
}

pub fn manipulate_text_buffer(key: char, text_buffer: &mut String) {
    match key {
        _ if key.is_ascii_whitespace() => {
            text_buffer.clear();
        }
        '\u{8}' => {
            text_buffer.pop();
        }
        _ => {
            text_buffer.push(key);
        }
    }
    sort_string(text_buffer);
}
