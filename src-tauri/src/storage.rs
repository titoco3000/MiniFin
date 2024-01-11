use futures::executor;
use serde::Serialize;
use sqlx::migrate::MigrateDatabase;
use sqlx::Pool;
use sqlx::Sqlite;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::tipos::*;
use crate::SqlDateTime;

//nao deve ser pub
pub fn obter_local_padrao() -> io::Result<PathBuf> {
    if let Some(mut local) = dirs_2::config_dir() {
        local.push("raja.toml");
        Ok(local)
    } else {
        io::Result::Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Sem diretorio \"config\"",
        ))
    }
}

#[derive(Serialize)]
pub struct Config {
    local_banco_de_dados: PathBuf,
}

impl Config {
    pub fn ler() -> io::Result<Config> {
        let local = obter_local_padrao()?;
        let string_toml = fs::read_to_string(local)?;
        let tabela_toml = string_toml.parse::<toml::Table>().unwrap();
        if let toml::Value::String(valor) = &tabela_toml["local_banco_de_dados"] {
            return Ok(Config {
                local_banco_de_dados: PathBuf::from(valor),
            });
        }
        io::Result::Err(io::Error::new(
            io::ErrorKind::NotFound,
            "campo \"local_banco_de_dados\" faltando ou mal-formatado em raja.toml",
        ))
    }
    pub fn salvar(&self) -> io::Result<()> {
        let local = obter_local_padrao()?;
        let string_toml = toml::to_string(self).unwrap();
        fs::write(local, &string_toml)
    }
}

//cria banco no local especificado, com os campos especificados
//Em um lugar padrão, armazena o lugar especificado
pub async fn criar_database(
    local_banco_de_dados: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = "sqlite://".to_string() + &local_banco_de_dados.to_str().unwrap();
    //se não existe
    if !sqlx::Sqlite::database_exists(&db_url)
        .await
        .unwrap_or(false)
    {
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
        let pool = sqlx::SqlitePool::connect(&db_url).await?;
        let qry = "PRAGMA foreign_keys = ON ;
        CREATE TABLE IF NOT EXISTS TiposDePagamento (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            nome                    TEXT UNIQUE NOT NULL,
            modificado              DATE DEFAULT (date('now','localtime'))
        );
        
        CREATE TABLE IF NOT EXISTS CaixasDeEntrada (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            nome                    TEXT UNIQUE NOT NULL,
            modificado              DATE DEFAULT (date('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS Empresas (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            nome                    TEXT UNIQUE NOT NULL,
            modificado              DATE DEFAULT (date('now','localtime'))
        );

        CREATE TABLE IF NOT EXISTS Setores (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            nome                    TEXT NOT NULL,
            id_empresa              INTEGER NOT NULL,
            modificado              DATE DEFAULT (date('now','localtime')),
            FOREIGN KEY (id_empresa) REFERENCES Empresas(id), 
            UNIQUE(nome, id_empresa)
        );

        CREATE TABLE IF NOT EXISTS Fornecedores (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            nome                    TEXT UNIQUE NOT NULL,
            id_setor                 INTEGER NOT NULL,
            id_tipo_pagamento         INTEGER NOT NULL,
            id_caixa                 INTEGER NOT NULL,
            modificado              DATE DEFAULT (date('now','localtime')),
            FOREIGN KEY (id_setor) REFERENCES Setores(id),
            FOREIGN KEY (id_tipo_pagamento) REFERENCES TiposDePagamento(id),
            FOREIGN KEY (id_caixa) REFERENCES CaixasDeEntrada(id)
        );

        CREATE TABLE IF NOT EXISTS Gastos (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            valor                   INTEGER NOT NULL,
            nf                      INTEGER NOT NULL,
            data                    DATE,
            id_setor                 INTEGER NOT NULL,
            id_caixa                 INTEGER NOT NULL,
            id_tipo_pagamento         INTEGER NOT NULL,
            id_fornecedor            INTEGER NOT NULL,
            obs               TEXT DEFAULT '',
            modificado              DATE DEFAULT (date('now','localtime')),
            FOREIGN KEY (id_setor) REFERENCES Setores(id),
            FOREIGN KEY (id_caixa) REFERENCES CaixasDeEntrada(id),
            FOREIGN KEY (id_tipo_pagamento) REFERENCES TiposDePagamento(id),
            FOREIGN KEY (id_fornecedor) REFERENCES Fornecedores(id)
        )";
        let _result = sqlx::query(&qry).execute(&pool).await?;
        pool.close().await;
        Config {
            local_banco_de_dados: PathBuf::from(local_banco_de_dados),
        }
        .salvar()?;
        Ok(())
    } else {
        simple_error::bail!("Já tem database lá");
    }
}

//cria ou modifica o arquivo de config no local-padrão para indicar onde fica o banco
//se o banco não estiver lá, retorna erro
pub fn localizar_database(local_banco_de_dados: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if local_banco_de_dados.exists() {
        let config = match Config::ler() {
            Ok(mut config) => {
                config.local_banco_de_dados = local_banco_de_dados;
                config
            }
            Err(_) => Config {
                local_banco_de_dados: local_banco_de_dados,
            },
        };
        config.salvar()?;
        Ok(())
    } else {
        simple_error::bail!("Sem banco de dados nesse local")
    }
}

pub struct BancoDeDados(Pool<Sqlite>);

impl BancoDeDados {
    //Lê do lugar-padrão onde está o banco
    //Abre a conexão e retorna ela
    //Se não encontra config no lugar-padrão ou não encontra banco, retorna o erro
    pub async fn abrir() -> Result<BancoDeDados, Box<dyn std::error::Error>> {
        let local_banco = Config::ler()?.local_banco_de_dados;
        let db_url = "sqlite://".to_string() + &local_banco.to_str().unwrap();
        let pool = sqlx::SqlitePool::connect(&db_url).await?;
        Ok(BancoDeDados(pool))
    }

    pub async fn obter_tipo_pagamento(&mut self, nome: &str) -> Option<TipoDePagamento> {
        match sqlx::query_as::<_, TipoDePagamento>(&format!(
            "SELECT * from TiposDePagamento
            WHERE nome = '{}'
            ",
            nome
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(pagamento) => Some(pagamento),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_caixa(&mut self, nome: &str) -> Option<CaixaDeEntrada> {
        match sqlx::query_as::<_, CaixaDeEntrada>(&format!(
            "SELECT * from CaixasDeEntrada
            WHERE nome = '{}'
            ",
            nome
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(caixa) => Some(caixa),
            Err(e) => {
                eprintln!("Erro na linha {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_empresa(&mut self, nome: &str) -> Option<Empresa> {
        match sqlx::query_as::<_, Empresa>(&format!(
            "SELECT * from Empresas
            WHERE nome = '{}'
            ",
            nome
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(empresa) => Some(empresa),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_setor(&mut self, nome: &str, nome_empresa: &str) -> Option<Setor> {
        match self.obter_empresa(nome_empresa).await {
            Some(empresa) => {
                match sqlx::query_as::<_, Setor>(&format!(
                    "
                    SELECT * from Setores
                    WHERE nome = '{}' AND id_empresa = {}
                    ",
                    nome, empresa.id
                ))
                .fetch_one(&self.0)
                .await
                {
                    Ok(empresa) => Some(empresa),
                    Err(e) => {
                        eprintln!("line {}: {}", line!(), e);
                        None
                    }
                }
            }
            None => None,
        }
    }
    pub async fn obter_fornecedor(&mut self, nome: &str) -> Option<Fornecedor> {
        match sqlx::query_as::<_, Fornecedor>(&format!(
            "SELECT * from Fornecedores
            WHERE nome = '{}'
            ",
            nome
        ))
        .fetch_optional(&self.0)
        .await
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }

    //metodos de obtenção por id
    pub async fn obter_tipo_pagamento_por_id(&mut self, id: u32) -> Option<TipoDePagamento> {
        match sqlx::query_as::<_, TipoDePagamento>(&format!(
            "SELECT * from TiposDePagamento
            WHERE id = {}",
            id
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(pagamento) => Some(pagamento),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_caixa_por_id(&mut self, id: u32) -> Option<CaixaDeEntrada> {
        match sqlx::query_as::<_, CaixaDeEntrada>(&format!(
            "SELECT * from CaixasDeEntrada
            WHERE id = {}",
            id
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(caixa) => Some(caixa),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_empresa_por_id(&mut self, id: u32) -> Option<Empresa> {
        match sqlx::query_as::<_, Empresa>(&format!(
            "SELECT * from Empresas
            WHERE id = {}",
            id
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(empresa) => Some(empresa),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_setor_por_id(&mut self, id: u32) -> Option<Setor> {
        match sqlx::query_as::<_, Setor>(&format!(
            "SELECT * from Setores
            WHERE id = {}
            ",
            id
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(empresa) => Some(empresa),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }
    pub async fn obter_fornecedor_por_id(&mut self, id: u32) -> Option<Fornecedor> {
        match sqlx::query_as::<_, Fornecedor>(&format!(
            "SELECT * from Fornecedores
            WHERE id = {}",
            id
        ))
        .fetch_one(&self.0)
        .await
        {
            Ok(empresa) => Some(empresa),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }

    pub async fn listar_caixas(&mut self) -> Vec<CaixaDeEntrada> {
        match sqlx::query_as::<_, CaixaDeEntrada>("SELECT * from CaixasDeEntrada")
            .fetch_all(&self.0)
            .await
        {
            Ok(caixas) => caixas,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn listar_empresas(&mut self) -> Vec<Empresa> {
        match sqlx::query_as::<_, Empresa>("SELECT * from Empresas")
            .fetch_all(&self.0)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn listar_fornecedores(&mut self) -> Vec<Fornecedor> {
        match sqlx::query_as::<_, Fornecedor>("SELECT * from Fornecedores")
            .fetch_all(&self.0)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }
    pub async fn listar_setores(&mut self) -> Vec<Setor> {
        match sqlx::query_as::<_, Setor>("SELECT * from Setores")
            .fetch_all(&self.0)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn listar_tipos_pagamento(&mut self) -> Vec<TipoDePagamento> {
        match sqlx::query_as::<_, TipoDePagamento>("SELECT * from TiposDePagamento")
            .fetch_all(&self.0)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn listar_gastos_filtrados(
        &mut self,
        data: &[&(SqlDateTime, SqlDateTime)],
        fornecedor: &[&Fornecedor],
        caixa: &[&CaixaDeEntrada],
        pagamento: &[&TipoDePagamento],
        setor: &[&Setor],
        empresa: &[&Empresa],
        pesquisa_obs: &[&str],
    ) -> Vec<Gasto> {
        let mut query = String::from("SELECT * from Gastos");
        let condicoes = "(".to_owned()
            + &([
                data.iter()
                    .map(|(inicio, fim)| format!("(data <= '{}' AND data >= '{}')", inicio, fim))
                    .collect::<Vec<String>>()
                    .join(" OR "),
                fornecedor
                    .iter()
                    .map(|x| format!("id_fornecedor = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR "),
                caixa
                    .iter()
                    .map(|x| format!("id_caixa = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR "),
                pagamento
                    .iter()
                    .map(|x| format!("idPagamento = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR "),
                if setor.is_empty() && empresa.is_empty() {
                    "".to_owned()
                } else if empresa.is_empty() {
                    setor
                        .iter()
                        .map(|x| format!("id_setor = {}", x.id))
                        .collect::<Vec<String>>()
                        .join(" OR ")
                } else if setor.is_empty() {
                    self.listar_setores()
                        .await
                        .iter()
                        .filter(|setor| {
                            empresa.iter().any(|empresa| empresa.id == setor.id_empresa)
                        })
                        .map(|x| format!("id_setor = {}", x.id))
                        .collect::<Vec<String>>()
                        .join(" OR ")
                } else {
                    setor
                        .iter()
                        .filter(|s| empresa.iter().any(|empresa| empresa.id == s.id_empresa))
                        .map(|x| format!("id_setor = {}", x.id))
                        .collect::<Vec<String>>()
                        .join(" OR ")
                },
            ]
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>()
            .join(") AND (")
                + ")");
        if !condicoes.is_empty() {
            query += " WHERE ";
            query += &condicoes;
        }

        match sqlx::query_as::<_, Gasto>(&query).fetch_all(&self.0).await {
            Ok(v) => v
                .into_iter()
                .filter(|gasto| {
                    pesquisa_obs
                        .iter()
                        .any(|pesquisado| gasto.obs.contains(pesquisado))
                })
                .collect(),
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn registrar_ou_atualizar_fornecedor(
        &mut self,
        nome: &str,
        setor_preferido: &Setor,
        pagamento_preferido: &TipoDePagamento,
        caixa_preferido: &CaixaDeEntrada,
    ) -> Result<Fornecedor, String> {
        let nome = nome.replace(&[',', ';'][..], "");
        if nome.trim().is_empty() {
            return Err("nome vazio".to_string());
        }
        println!("vendo se fornecedor ja existe");
        match self.obter_fornecedor(&nome).await {
            Some(f) => {
                //se info está diferente
                if f.id_setor != setor_preferido.id
                    || f.id_tipo_pagamento != pagamento_preferido.id
                    || f.id_caixa != caixa_preferido.id
                {
                    println!("Fornecedor existe! Atualizando...");
                    sqlx::query(&format!(
                        "UPDATE Fornecedores SET 
                                id_setor = {},
                                id_tipo_pagamento = {},
                                id_caixa = {},
                                modificado = (date('now','localtime')
                            WHERE id = {};
                            ",
                        setor_preferido.id, pagamento_preferido.id, caixa_preferido.id, f.id
                    ))
                    .execute(&self.0)
                    .await
                    .unwrap();
                }
                Ok(f)
            }
            None => {
                println!("Fornecedor nao existe");
                match sqlx::query_as::<_, Fornecedor>(&format!(
                    "INSERT INTO Fornecedores 
                            (nome, id_setor, id_tipo_pagamento, id_caixa)
                            VALUES 
                            ('{}',{},{},{})
                            RETURNING *
                        ",
                    nome, setor_preferido.id, pagamento_preferido.id, caixa_preferido.id
                ))
                .fetch_one(&self.0)
                .await
                {
                    Ok(f) => Ok(f),
                    Err(e) => {
                        println!("e (line {}): {}", line!(), e);
                        Err(e.to_string())
                    }
                }
            }
        }
    }

    //salva o gasto
    pub async fn registrar_gasto(
        &mut self,
        valor: u32,
        nf: u32,
        data: SqlDateTime,
        setor: &Setor,
        caixa: &CaixaDeEntrada,
        tipo_pagamento: &TipoDePagamento,
        fornecedor: &Fornecedor,
        obs: String,
    ) -> Result<(), Vec<String>> {
        println!("Registrar gasto()");
        let mut problemas = Vec::with_capacity(9);

        if self.obter_gasto(&fornecedor, nf).await.is_some() {
            problemas.push(String::from("nf já ultilizada para esse fornecedor"));
        }

        if problemas.len() == 0 {
            println!("sem problemas até aqui");
            match sqlx::query(&format!(
                "INSERT INTO Gastos 
                    (valor, nf, data, id_setor, id_caixa, id_tipo_pagamento, id_fornecedor, obs)
                    VALUES
                    ({},{},'{}',{},{},{},{},'{}')
                    ",
                valor as i32,
                nf as i32,
                data,
                setor.id,
                caixa.id,
                tipo_pagamento.id,
                fornecedor.id,
                obs
            ))
            .execute(&self.0)
            .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    problemas.push(format!("Erro SQL: {}", e));
                    println!("Erro SQL: {}", e);
                    Err(problemas)
                }
            }
        } else {
            Err(problemas)
        }
    }

    /*
       "nenhum valor recebido
       nenhum caixa recebido
       formato de data incorreto
       nenhuma nf recebida"
    */
    pub async fn registrar_gasto_de_json(&mut self, json_str: &str) -> Result<(), Vec<String>> {
        println!("Recebi json: {}", json_str);
        let mut problemas = Vec::with_capacity(9);

        if let Ok(json_obj) = serde_json::from_str::<serde_json::Value>(json_str) {
            let valor = match &json_obj["valor"] {
                serde_json::Value::Number(val) => Some(val.as_u64().unwrap() as u32),
                serde_json::Value::String(str_val)=>{
                    match str_val.replace(".", "").replace(",", "").parse::<u32>() {
                        Ok(val)=>Some(val),
                        Err(_)=> {
                            problemas.push(String::from("valor mal-formatado"));
                            None
                        }
                    }
                }
                serde_json::Value::Null => {
                    problemas.push(String::from("nenhum valor recebido"));
                    None
                }
                _ => {
                    problemas.push(String::from("valor mal-formatado"));
                    None
                }
            };

            let caixa = match &json_obj["caixa"] {
                serde_json::Value::String(caixa_str)=>{
                    if let Some(v) = self.obter_caixa(caixa_str).await {
                        Some(v)
                    } else {
                        problemas.push(String::from("caixa não encontrado"));
                        None
                    }
                }
                serde_json::Value::Null=>{
                    println!("é null");
                    problemas.push(String::from("nenhum caixa recebido"));
                    None    
                }
                _=>{
                    problemas.push(String::from("caixa mal-formatado"));
                    None    
                }
            };

            let setor = if let serde_json::Value::String(empresa_str) = &json_obj["empresa"] {
                if let serde_json::Value::String(setor_str) = &json_obj["setor"] {
                    if let Some(_) = self.obter_empresa(&empresa_str).await {
                        if let Some(v) = self.obter_setor(&setor_str, &empresa_str).await {
                            Some(v)
                        } else {
                            problemas.push(String::from("setor não encontrado nessa empresa"));
                            None
                        }
                    } else {
                        problemas.push(String::from("empresa não encontrada"));
                        None
                    }
                } else {
                    problemas.push(String::from("nenhum setor recebido"));
                    None
                }
            } else {
                problemas.push(String::from("nenhuma empresa recebida"));
                None
            };

            let tipo_pagamento =
                if let serde_json::Value::String(pagamento_str) = &json_obj["pagamento"] {
                    if let Some(v) = self.obter_tipo_pagamento(&pagamento_str).await {
                        Some(v)
                    } else {
                        problemas.push(String::from("tipo de pagamento não encontrado"));
                        None
                    }
                } else {
                    problemas.push(String::from("nenhum tipo de pagamento recebido"));
                    None
                };

            let data = if let serde_json::Value::String(data_str) = &json_obj["data"] {
                match data_str.parse::<SqlDateTime>() {
                    Ok(v) => {
                        Some(v)
                    }
                    Err(_) => {
                        problemas.push(String::from("formato de data incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhuma data recebida"));
                None
            };

            let fornecedor =
                if let serde_json::Value::String(fornecedor_str) = &json_obj["fornecedor"] {
                    if setor.is_some() && tipo_pagamento.is_some() && caixa.is_some() {
                        match self
                            .registrar_ou_atualizar_fornecedor(
                                &fornecedor_str,
                                &setor.clone().unwrap(),
                                &tipo_pagamento.clone().unwrap(),
                                &caixa.clone().unwrap(),
                            )
                            .await
                        {
                            Ok(v) => Some(v),
                            Err(e) => {
                                eprintln!("Erro ao reg/att fornecedor: {}", e);
                                problemas.push(e);
                                None
                            }
                        }
                    } else {
                        //problemas já vão ter sido reportados antes
                        None
                    }
                } else {
                    problemas.push(String::from("nenhum fornecedor recebido"));
                    None
                };

            let nf = match &json_obj["nf"] {
                serde_json::Value::Number(val) => Some(val.as_u64().unwrap() as u32),
                serde_json::Value::String(str_val)=>{
                    match str_val.parse::<u32>() {
                        Ok(val)=>Some(val),
                        Err(_)=> {
                            problemas.push(String::from("nf mal-formatada"));
                            None
                        }
                    }
                }
                serde_json::Value::Null => {
                    problemas.push(String::from("nenhuma nf recebida"));
                    None
                }
                _ => {
                    problemas.push(String::from("nf mal-formatada"));
                    None
                }
            };

            let obs = if let serde_json::Value::String(obs_str) = &json_obj["obs"] {
                obs_str.replace(";", "")
            } else {
                String::from("")
            };

            if valor.is_some()
                && nf.is_some()
                && data.is_some()
                && caixa.as_ref().is_some()
                && tipo_pagamento.is_some()
                && fornecedor.is_some()
            {
                return self
                    .registrar_gasto(
                        valor.unwrap(),
                        nf.unwrap(),
                        data.unwrap(),
                        &setor.unwrap(),
                        &caixa.unwrap(),
                        &tipo_pagamento.unwrap(),
                        &fornecedor.unwrap(),
                        obs,
                    )
                    .await;
            }
        }
        Err(problemas)
    }

    pub async fn validar_gasto_de_json(&mut self, json_str: &str) -> Result<(), Vec<String>> {
        let mut problemas = Vec::with_capacity(9);
        if let Ok(json_obj) = serde_json::from_str::<serde_json::Value>(json_str) {
            let _valor = if let serde_json::Value::String(valor_str) = &json_obj["valor"] {
                match valor_str.replace(".", "").replace(",", "").parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        problemas.push(String::from("formato do valor incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhum valor recebido"));
                None
            };

            let caixa = if let serde_json::Value::String(caixa_str) = &json_obj["caixa"] {
                if let Some(v) = self.obter_caixa(caixa_str).await {
                    Some(v)
                } else {
                    problemas.push(String::from("caixa não encontrado"));
                    None
                }
            } else {
                problemas.push(String::from("nenhum caixa recebido"));
                None
            };

            let setor = if let serde_json::Value::String(empresa_str) = &json_obj["empresa"] {
                if let serde_json::Value::String(setor_str) = &json_obj["setor"] {
                    if let Some(_) = self.obter_empresa(&empresa_str).await {
                        if let Some(v) = self.obter_setor(&setor_str, &empresa_str).await {
                            Some(v)
                        } else {
                            problemas.push(String::from("setor não encontrado nessa empresa"));
                            None
                        }
                    } else {
                        problemas.push(String::from("empresa não encontrada"));
                        None
                    }
                } else {
                    problemas.push(String::from("nenhum setor recebido"));
                    None
                }
            } else {
                problemas.push(String::from("nenhuma empresa recebida"));
                None
            };

            let tipo_pagamento =
                if let serde_json::Value::String(pagamento_str) = &json_obj["tipo_pagamento"] {
                    if let Some(v) = self.obter_tipo_pagamento(&pagamento_str).await {
                        Some(v)
                    } else {
                        problemas.push(String::from("tipo de pagamento não encontrado"));
                        None
                    }
                } else {
                    problemas.push(String::from("nenhum caixa recebido"));
                    None
                };

            let _data = if let serde_json::Value::String(data_str) = &json_obj["data"] {
                match data_str.parse::<SqlDateTime>() {
                    Ok(v) => {
                        println!("talvez seria bom alguma verificação de data...?");
                        Some(v)
                    }
                    Err(_) => {
                        problemas.push(String::from("formato de data incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhuma data recebida"));
                None
            };

            let nf = if let serde_json::Value::String(nf_str) = &json_obj["nf"] {
                match nf_str.parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        problemas.push(String::from("formato da nf incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhuma nf recebida"));
                None
            };

            let _fornecedor: Option<Fornecedor> = if let serde_json::Value::String(fornecedor_str) =
                &json_obj["fornecedor"]
            {
                if setor.is_some() && tipo_pagamento.is_some() && caixa.is_some() && nf.is_some() {
                    match self.obter_fornecedor(&fornecedor_str).await {
                        //se esse fornecedor existe
                        Some(v) => {
                            if let Some(_) = self.obter_gasto(&v, nf.unwrap()).await {
                                problemas
                                    .push("Nota fiscal já usada por esse fornecedor".to_string());
                            }
                            None
                        }
                        None => None,
                    }
                } else {
                    //problemas já vão ter sido reportados antes
                    None
                }
            } else {
                problemas.push(String::from("nenhum fornecedor recebido"));
                None
            };
        }
        return if problemas.len() == 0 {
            Ok(())
        } else {
            Err(problemas)
        };
    }

    pub async fn registrar_basicos(
        &mut self,
        caixas: &[&str],
        empresas: &[&str],
        setores: &[(&str, &str)],
        pagamentos: &[&str],
    ) {
        for caixa in caixas {
            sqlx::query(&format!(
                "INSERT INTO CaixasDeEntrada 
                    (nome)
                    VALUES
                    ('{}');
                    ",
                caixa
            ))
            .execute(&self.0)
            .await
            .expect("Erro ao inserir caixa");
        }
        for empresa in empresas {
            sqlx::query(&format!(
                "INSERT INTO Empresas 
                    (nome)
                    VALUES
                    ('{}');
                    ",
                empresa
            ))
            .execute(&self.0)
            .await
            .expect("Erro ao inserir empresa");
        }
        for (setor, empresa) in setores {
            let id_empresa = self
                .obter_empresa(empresa)
                .await
                .expect("Tentando adicionar setor a empresa inexistente")
                .id;
            sqlx::query(&format!(
                "INSERT INTO Setores 
                    (nome, id_empresa)
                    VALUES
                    ('{}', {});
                    ",
                setor, id_empresa
            ))
            .execute(&self.0)
            .await
            .expect("Erro ao inserir setor");
        }
        for pagamento in pagamentos {
            sqlx::query(&format!(
                "INSERT INTO TiposDePagamento 
                    (nome)
                    VALUES
                    ('{}');
                    ",
                pagamento
            ))
            .execute(&self.0)
            .await
            .expect("Erro ao inserir tipo de pagamento");
        }
    }

    pub async fn obter_gasto(&mut self, fornecedor: &Fornecedor, nf: u32) -> Option<Gasto> {
        match sqlx::query_as::<_, Gasto>(&format!(
            "SELECT * FROM Gastos WHERE 
            nf = {} AND id_fornecedor = {}",
            nf, fornecedor.id
        ))
        .fetch_optional(&self.0)
        .await
        {
            Ok(gasto) => gasto,
            Err(e) => {
                eprintln!("line {}: {}", line!(), e);
                None
            }
        }
    }

    //dada identificação, remove gasto do banco
    pub async fn remover_gasto(
        &mut self,
        nf: u32,
        nome_fornecedor: &str,
    ) -> Result<(), Box<dyn Error>> {
        match self.obter_fornecedor(nome_fornecedor).await {
            Some(f) => {
                match sqlx::query(&format!(
                    "DELETE FROM Gastos WHERE 
                    nf = {} AND id_fornecedor = {}",
                    nf, f.id
                ))
                .execute(&self.0)
                .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            None => Err("fornecedor não encontrado")?,
        }
    }
    //dado o nome, remove o fornecedor e move todas as gastos dele
    // para algum outro fornecedor especificado
    pub async fn remover_fornecedor(&mut self, original: &Fornecedor, novo: &Fornecedor) {
        sqlx::query(&format!(
            "UPDATE Gastos SET 
                    id_fornecedor = {},
                    modificado = (date('now','localtime')
                WHERE id_fornecedor = {};
                ",
            novo.id, original.id
        ))
        .execute(&self.0)
        .await
        .unwrap();
        sqlx::query(&format!(
            "DELETE from Fornecedores 
                WHERE id = {};
                ",
            original.id
        ))
        .execute(&self.0)
        .await
        .unwrap();
    }
}

impl Drop for BancoDeDados {
    fn drop(&mut self) {
        executor::block_on(self.0.close());
    }
}
