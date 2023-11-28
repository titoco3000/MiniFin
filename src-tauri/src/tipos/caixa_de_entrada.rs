use crate::SqlDateTime;
use serde::Serialize;

#[derive(Serialize, Clone, sqlx::FromRow, Debug)]
pub struct CaixaDeEntrada {
    pub id: u32,
    pub nome: String,
    pub modificado: SqlDateTime,
}
