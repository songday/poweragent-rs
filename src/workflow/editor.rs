use std::vec::Vec;

use axum::extract::Query;
use axum::Json;
use axum::response::IntoResponse;

use super::dto::Workflow;
use crate::db;
use crate::db_executor;
use crate::util::Result;
use crate::web::server::to_res;

pub(crate) const TABLE_SUFFIX: &str = "workflows";

pub(crate) fn init() -> Result<()> {
    // let table_name = format!("{}-mainflows", robot_id);
    // let main_flow_table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    // db::init_table(main_flow_table)?;
    db_executor!(db::init_table, "", TABLE_SUFFIX,)?;
    // let table_name = format!("{}-subflows", robot_id);
    // let sub_flow_table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    // db::init_table(sub_flow_table)?;
    Ok(())
}

pub(crate) async fn list() -> impl IntoResponse {
    let r: Result<Vec<Workflow>> = db_executor!(db::get_all, "", TABLE_SUFFIX,);
    to_res(r)
}

pub(crate) async fn create(Query(d) : Query<Workflow>) -> impl IntoResponse {
    let workflow = Workflow {
        id: scru128::new_string(),
        name: d.name.clone(),
        canvas: String::new(),
    };
    match db_executor!(db::write, &workflow.id, TABLE_SUFFIX, &workflow.id, &workflow) {
        Ok(_) => {
            to_res(Ok(workflow))
        }
        Err(e) => {
            to_res(Err(e))
        }
    }
}

pub(crate) async fn get(Query(d) : Query<Workflow>) -> impl IntoResponse {
    let w: Result<Option<Workflow>> = db_executor!(db::query, "", TABLE_SUFFIX, d.id.as_str());
    to_res(w)
}

pub(crate) async fn save(Json(mut data): Json<Workflow>) -> impl IntoResponse {
    to_res(db_executor!(db::write, &data.id, TABLE_SUFFIX, &data.id, &data))
}

pub(crate) async fn delete(Query(d) : Query<Workflow>) -> impl IntoResponse {
    to_res(db_executor!(db::remove, "", TABLE_SUFFIX, d.id.as_str()))
}
