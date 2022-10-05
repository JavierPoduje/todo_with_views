use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;
use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title: String = req.match_info().get("title").unwrap().to_string();

    let title_reference: String = title.clone();

    let item = to_do::to_do_factory(&String::from("pending"), title).expect("create");

    process_input(item, "create".to_string(), &state);

    return format!("{} created", title_reference);
}
