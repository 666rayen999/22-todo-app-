use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("New Project")
        .append_to_head(include_str!("../favicon.html"))
        .append_to_head("<style>:root,html,body{height:100%;}body{background-color:black;}.space-between{justify-content:space-between;}</style>")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
