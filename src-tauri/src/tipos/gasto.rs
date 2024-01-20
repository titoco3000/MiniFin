use crate::{storage::BancoDeDados, SqlDateTime};
use serde::Serialize;
use serde_json;

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

impl Gasto {
    pub async fn unpack(
        &self,
        database: &mut BancoDeDados,
    ) -> Result<serde_json::Value, &'static str> {
        if let Some(setor) = database.obter_setor_por_id(self.id_setor).await {
            let resultado = serde_json::json!({
             "valor": self.valor,
             "nf": self.nf,
             "data": self.data,
            "setor": setor.nome,
            "empresa": if let Some(v) = database.obter_empresa_por_id(setor.id_empresa).await{v.nome} else{return Err("Empresa não localizada");},
            "caixa": if let Some(v) = database.obter_caixa_por_id(self.id_caixa).await{v.nome} else{return Err("Caixa não localizado");},
            "pagamento": if let Some(v) = database.obter_tipo_pagamento_por_id(self.id_tipo_pagamento).await{v.nome} else{return Err("Tipo de pagamento não localizado");},
            "fornecedor": if let Some(v) = database.obter_fornecedor_por_id(self.id_fornecedor).await{v.nome} else{return Err("Fornecedor não localizado");},
            "obs": self.obs,
            });
            Ok(resultado)
        } else {
            Err("Setor não localizado")
        }
    }
}
