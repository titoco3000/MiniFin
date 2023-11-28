use crate::SqlDateTime;
use serde::Serialize;

#[derive(Serialize, Clone, sqlx::FromRow, Debug)]
pub struct Gasto {
    pub id: u32,
    pub valor: u32,
    pub nf: u32,
    pub data: SqlDateTime,
    pub id_setor: u32,
    pub id_caixa: u32,
    pub id_tipo_pagamento: u32,
    pub id_fornecedor: u32,
    pub obs: String,
    pub modificado: SqlDateTime,
}
