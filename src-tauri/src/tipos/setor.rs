use crate::SqlDateTime;
use serde::Serialize;

#[derive(Serialize, Clone, sqlx::FromRow, Debug)]
pub struct Setor {
    pub id: u32,
    pub nome: String,
    pub id_empresa: u32,
    pub modificado: SqlDateTime,
}
