mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let usr_req: String = get_user_response("What are we building Today");
    dbg!(usr_req);
}