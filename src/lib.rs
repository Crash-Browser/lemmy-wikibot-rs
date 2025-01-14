use std::fs;
use std::path::Path;

use structs::CustomWikipediaPage;

pub mod apis;
pub mod structs;

pub fn comment_builder(wiki_page: CustomWikipediaPage) -> String {
    format!(
        "Here's the {} for the wikipedia article you mentioned in your comment:

`
{}
`

[^article^](https://en.wikipedia.org/wiki/{}) ^|^ [^about^](https://lemmy.world/u/wikibot)
",
        if wiki_page.is_section {
            "section"
        } else {
            "summary"
        },
        wiki_page.content.trim(),
        wiki_page.page_title,
    )
}

pub fn save_to_db(comment_id: Option<u32>, manual: Option<Vec<u32>>) {
    let vec_path = Path::new("checked_comments.json");
    if vec_path.exists() {
        if let Some(id) = comment_id {
            let file_content = fs::read_to_string(vec_path).unwrap();
            let mut deserialized_vec = serde_json::from_str::<Vec<u32>>(&file_content).unwrap();
            deserialized_vec.push(id);
            fs::write(vec_path, serde_json::to_string(&deserialized_vec).unwrap()).unwrap();
        } else {
            // manual is supplied
            let new_vec = manual.unwrap();
            fs::write(vec_path, serde_json::to_string(&new_vec).unwrap()).unwrap();
        }
    } else {
        panic!("checked_comments.json does not exist!")
    }
}

pub fn load_db() -> Vec<u32> {
    let vec_path = Path::new("checked_comments.json");
    if !vec_path.exists() {
        println!("checked_comments.json file does not exist, creating it...");
        fs::write(vec_path, serde_json::to_vec(&vec![0_u32]).unwrap()).unwrap();
        Vec::new().push(0)
    }
    let file_content = fs::read_to_string(vec_path).unwrap();

    serde_json::from_str::<Vec<u32>>(&file_content).unwrap()
}
