use crate::model::Story;
use toasty::Db;

pub async fn connect(database_url: &str) -> toasty::Result<Db> {
    Db::builder()
        .models(toasty::models!(Story))
        .connect(database_url)
        .await
}
