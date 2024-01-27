use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct FiltroGasto {
    pub data_inicial: Vec<String>,
    pub data_final: Vec<String>,
    pub setor: Vec<String>,
    pub empresa: Vec<String>,
    pub caixa: Vec<String>,
    pub tipo_pagamento: Vec<String>,
    pub fornecedor: Vec<String>,
    pub obs_pesquisa: Vec<String>,
    pub conteudo: Vec<String>
}
