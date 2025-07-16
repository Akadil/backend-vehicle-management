pub struct CreateTemplateCommand {
    pub string: String,
    pub integer: i32,
    pub float: f32,
}

pub struct CreateTemplateResponse {
    pub id: String,
    pub message: String,
}