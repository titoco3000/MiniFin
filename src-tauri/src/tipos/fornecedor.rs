use crate::SqlDateTime;
use serde::Serialize;

#[derive(Serialize, Clone, sqlx::FromRow, Debug)]
pub struct Fornecedor {
    pub id: u32,
    pub nome: String,
    pub id_setor: u32,
    pub id_tipo_pagamento: u32,
    pub id_caixa: u32,
    pub modificado: SqlDateTime,
}
