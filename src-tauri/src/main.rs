#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod org_parser;

use org_parser::parse_org_document;

#[tauri::command]
fn parse_org_content(content: String) -> Result<String, String> {
    match parse_org_document(&content) {
        Ok(doc) => Ok(format!("{:#?}", doc)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn test_parser() -> Result<String, String> {
    let test_content = r#"* Top Level Heading :tag1:tag2:
Some paragraph text here.
It can span multiple lines.

** Second Level :important:
- First bullet point
- Second bullet point
  
*** Third Level
1. Ordered item 1
2. Ordered item 2

* Another Top Level
More content here.
"#;

    match parse_org_document(test_content) {
        Ok(doc) => Ok(format!("{:#?}", doc)),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_org_content, test_parser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
