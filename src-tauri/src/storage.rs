use chrono::Datelike;
use futures::executor;

use serde::Serialize;
use sqlx::Row;

use crate::custom_console::macros as console;
use crate::tipos::*;
use crate::SqlDateTime;
use sqlx::migrate::MigrateDatabase;
use sqlx::Pool;
use sqlx::Sqlite;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

//nao deve ser pub
pub fn obter_local_padrao() -> io::Result<PathBuf> {
    if let Some(mut local) = dirs_2::config_dir() {
        local.push("minifin.toml");
        Ok(local)
    } else {
        io::Result::Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Sem diretorio \"config\"",
        ))
    }
}

fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}

#[derive(Serialize)]
pub struct Config {
    local_banco_de_dados: PathBuf,
}

impl Config {
    pub fn new(local_banco_de_dados: PathBuf) -> Config {
        Config {
            local_banco_de_dados: local_banco_de_dados,
        }
    }
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
            "campo \"local_banco_de_dados\" faltando ou mal-formatado em minifin.toml",
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
        //garante que a pasta /minifin/ exista
        let mut folder = local_banco_de_dados.clone();
        folder.pop();
        std::fs::create_dir_all(folder)?;

        sqlx::Sqlite::create_database(&db_url).await?;
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
        simple_error::bail!("Já tem database em {}", &db_url);
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

        //teste funciona!

        Ok(BancoDeDados(pool))
    }

    pub async fn obter_tipo_pagamento(&mut self, nome: &str) -> Option<TipoDePagamento> {
        match sqlx::query_as::<_, TipoDePagamento>(&format!(
            "SELECT * from TiposDePagamento
            WHERE nome = '{}'
            ",
            nome
        ))
        .fetch_optional(&self.0)
        .await
        {
            Ok(pagamento) => pagamento,
            Err(e) => {
                console::bad!("line {}: {}", line!(), e);
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
        .fetch_optional(&self.0)
        .await
        {
            Ok(caixa) => caixa,
            Err(e) => {
                console::bad!("Erro na linha {}: {}", line!(), e);
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
        .fetch_optional(&self.0)
        .await
        {
            Ok(empresa) => empresa,
            Err(e) => {
                console::bad!("line {}: {}", line!(), e);
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
                .fetch_optional(&self.0)
                .await
                {
                    Ok(empresa) => empresa,
                    Err(e) => {
                        console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
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
                console::bad!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }
    
    pub async fn contar_gastos(&mut self, filtro: &FiltroGasto)->u32{
        let mut query = String::from("SELECT COUNT(*) from Gastos");
        let condicoes = &self.filtro_into_query(filtro).await;

        let mut joiner = "";
        if let Some(c) = filtro.conteudo.get(0){
            if !c.is_empty(){
                joiner = "
                LEFT JOIN Fornecedores ON Gastos.id_fornecedor = Fornecedores.id
                LEFT JOIN Setores ON Gastos.id_setor = Setores.id 
                LEFT JOIN Empresas ON Setores.id = Empresas.id
                LEFT JOIN TiposDePagamento ON Gastos.id_tipo_pagamento = TiposDePagamento.id
                LEFT JOIN CaixasDeEntrada ON Gastos.id_caixa = CaixasDeEntrada.id
                ";
            }
        }

        if !condicoes.is_empty() {
            query += joiner;
            query += " WHERE (";
            query += &condicoes;
            query += " )";
        }
        console::regular!("{}",&query);

        sqlx::query(
            &query
        )
        .fetch_one(&self.0)
        .await.expect("Erro ao contar").get::<u32,usize>(0)
    }
    
    pub async fn somar_gastos(&mut self, filtro: &FiltroGasto)->u32{
        let mut query = String::from("SELECT sum(valor) from Gastos");
        let condicoes = &self.filtro_into_query(filtro).await;

        let mut joiner = "";
        if let Some(c) = filtro.conteudo.get(0){
            if !c.is_empty(){
                joiner = "
                LEFT JOIN Fornecedores ON Gastos.id_fornecedor = Fornecedores.id
                LEFT JOIN Setores ON Gastos.id_setor = Setores.id 
                LEFT JOIN Empresas ON Setores.id = Empresas.id
                LEFT JOIN TiposDePagamento ON Gastos.id_tipo_pagamento = TiposDePagamento.id
                LEFT JOIN CaixasDeEntrada ON Gastos.id_caixa = CaixasDeEntrada.id
                ";
            }
        }

        if !condicoes.is_empty() {
            query += joiner;
            query += " WHERE (";
            query += &condicoes;
            query += " )";
        }

        console::regular!("Somando de: {}",query);

        sqlx::query(
            &query
        )
        .fetch_one(&self.0)
        .await.expect("Erro ao somar").get::<u32,usize>(0)

    }
    pub async fn filtro_into_query(&mut self, filtro: &FiltroGasto)->String{
        
        let mut data: Vec<(Option<SqlDateTime>, Option<SqlDateTime>)> = Vec::with_capacity(1);
        let mut fornecedor: Vec<Fornecedor> = Vec::with_capacity(1);
        let mut caixa: Vec<CaixaDeEntrada> = Vec::with_capacity(1);
        let mut pagamento: Vec<TipoDePagamento> = Vec::with_capacity(1);
        let mut setor: Vec<Setor> = Vec::with_capacity(1);
        let mut empresa: Vec<Empresa> = Vec::with_capacity(1);
        let mut pesquisa_obs: Vec<&str> = Vec::with_capacity(1);
        let mut conteudo: Vec<&str> = Vec::with_capacity(1);

        //data
        for i in 0usize..std::cmp::max(filtro.data_inicial.len(), filtro.data_final.len()) {
            let data_range: (Option<SqlDateTime>, Option<SqlDateTime>) = (
                if let Some(str_date) = filtro.data_inicial.get(i) {
                    match SqlDateTime::parse_from_str(&str_date, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(_) => None,
                    }
                } else {
                    None
                },
                if let Some(str_date) = filtro.data_final.get(i) {
                    match SqlDateTime::parse_from_str(&str_date, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(_) => None,
                    }
                } else {
                    None
                },
            );
            data.push(data_range);
        }

        //fornecedor
        for nome in &filtro.fornecedor {
            if let Some(valor) = self.obter_fornecedor(nome).await {
                fornecedor.push(valor);
            }
            else {
                console::alert!("Fornecedor não reconhecido: {}",nome);
            }
        }

        //caixa
        for nome in &filtro.caixa {
            if let Some(valor) = self.obter_caixa(nome).await {
                caixa.push(valor);
            }
        }

        //pagamento
        for nome in &filtro.tipo_pagamento {
            if let Some(valor) = self.obter_tipo_pagamento(nome).await {
                pagamento.push(valor);
            }
        }

        //empresa
        for nome in &filtro.empresa {
            if let Some(valor) = self.obter_empresa(nome).await {
                empresa.push(valor);
            }
        }

        //setor
        for i in 0..filtro.setor.len() / 2 {
            if let Some(valor) = self
                .obter_setor(&filtro.setor[i + 1], &filtro.setor[i])
                .await
            {
                setor.push(valor);
            }
        }

        //obs
        for obs in &filtro.obs_pesquisa {
            if !obs.is_empty() {
                pesquisa_obs.push(obs);
            }
        }

        //conteudo
        for c in &filtro.conteudo {
            if !c.is_empty() {
                conteudo.push(c);
            }
        }

        [
            data.iter()
                .map(|(inicio, fim)| {
                    if inicio.is_some() && fim.is_some() {
                        format!(
                            "(data >= '{}' AND data <= '{}')",
                            inicio.unwrap(),
                            fim.unwrap()
                        )
                    } else if inicio.is_some() {
                        format!("(Gastos.data >= '{}')", inicio.unwrap())
                    } else if fim.is_some() {
                        format!("(Gastos.data <= '{}')", fim.unwrap())
                    } else {
                        "".to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join(" OR "),
            fornecedor
                .iter()
                .map(|x| format!("Gastos.id_fornecedor = {}", x.id))
                .collect::<Vec<String>>()
                .join(" OR "),
            caixa
                .iter()
                .map(|x| format!("Gastos.id_caixa = {}", x.id))
                .collect::<Vec<String>>()
                .join(" OR "),
            pagamento
                .iter()
                .map(|x| format!("Gastos.id_tipo_pagamento = {}", x.id))
                .collect::<Vec<String>>()
                .join(" OR "),
            if setor.is_empty() && empresa.is_empty() {
                "".to_owned()
            } else if empresa.is_empty() {
                setor
                    .iter()
                    .map(|x| format!("Gastos.id_setor = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR ")
            } else if setor.is_empty() {
                self.listar_setores()
                    .await
                    .iter()
                    .filter(|setor| empresa.iter().any(|empresa| empresa.id == setor.id_empresa))
                    .map(|x| format!("Gastos.id_setor = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR ")
            } else {
                setor
                    .iter()
                    .filter(|s| empresa.iter().any(|empresa| empresa.id == s.id_empresa))
                    .map(|x| format!("Gastos.id_setor = {}", x.id))
                    .collect::<Vec<String>>()
                    .join(" OR ")
            },
            pesquisa_obs
            .iter()
            .map(|x| format!("obs like '%{}%'",x))
            .collect::<Vec<String>>()
            .join(" OR "),
            conteudo
                .iter()
                .map(|x| format!("
                strftime('%d/%m/%Y', Gastos.data) like '%{}%' OR
                Fornecedores.nome like '%{}%' OR
                Setores.nome like '%{}%' OR
                REPLACE(REPLACE(printf('%,d;%02d', Gastos.valor/100, Gastos.valor%100), ',','.'),';',',') like '%{}%' OR
                Empresas.nome like '%{}%' OR
                TiposDePagamento.nome like '%{}%' OR
                Gastos.nf like '%{}%' OR
                CaixasDeEntrada.nome like '%{}%' OR
                Gastos.obs like '%{}%'
                ", x,x,x,x,x,x,x,x,x))
                .collect::<Vec<String>>()
                .join(" OR "),
        ]
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>()
        .join(") AND (")
    }

    pub async fn listar_gastos_filtrados(
        &mut self,
        filtro: &FiltroGasto,
        limit: Option<u32>,
        offset: Option<u32>,
        sorter:&SortParameter
    ) -> Vec<Gasto> {
        let mut query = String::from("SELECT Gastos.* from Gastos ");
        let condicoes = &self.filtro_into_query(filtro).await;

        /*
        'Data',
		'Fornecedor',
		'Empresa',
		'Setor',
		'Valor',
		'Pagamento',
		'NF',
		'Caixa',
		'Observações'
         */
        let (mut joiner, order_by) = 
        match sorter.i {
            0 => ("", "Gastos.data"),
            1 => ("JOIN Fornecedores ON Gastos.id_fornecedor = Fornecedores.id","Fornecedores.nome"),
            2 => ("JOIN Setores ON Gastos.id_setor = Setores.id JOIN Empresas ON Setores.id = Empresas.id","Empresas.nome"),
            3 => ("JOIN Setores ON Gastos.id_setor = Setores.id","Setores.nome"),
            4 => ("","Gastos.valor"),
            5 => ("JOIN TiposDePagamento ON Gastos.id_tipo_pagamento = TiposDePagamento.id","TiposDePagamento.nome"),
            6 => ("","Gastos.nf"),
            7 => ("JOIN CaixasDeEntrada ON Gastos.id_caixa = CaixasDeEntrada.id","CaixasDeEntrada.nome"),
            8 => ("","Gastos.obs"),
            _ => panic!("ordenamento não planejado")
        };

        if let Some(c) = filtro.conteudo.get(0){
            if !c.is_empty(){
                joiner = "
                LEFT JOIN Fornecedores ON Gastos.id_fornecedor = Fornecedores.id
                LEFT JOIN Setores ON Gastos.id_setor = Setores.id 
                LEFT JOIN Empresas ON Setores.id = Empresas.id
                LEFT JOIN TiposDePagamento ON Gastos.id_tipo_pagamento = TiposDePagamento.id
                LEFT JOIN CaixasDeEntrada ON Gastos.id_caixa = CaixasDeEntrada.id
                ";
            }
        }
        query+=joiner;

        if !condicoes.is_empty() {
            query += " WHERE (";
            query += &condicoes;
            query += " )";
        }
        
        query+=" ORDER BY ";
        query+=order_by;
        query+=" COLLATE NOCASE";
        if sorter.d{
            query+=" DESC";
        }

        if let Some(lim) = limit{
            query += &format!(" LIMIT {}", lim);
        }
        if let Some(off) = offset{
            query += &format!(" OFFSET {}", off);
        }

        console::regular!("Pedindo: \n\"{}\"", query);

        match sqlx::query_as::<_, Gasto>(&query).fetch_all(&self.0).await {
            Ok(v) => v,
            Err(e) => {
                console::bad!("line {}: {}", line!(), e);
                Vec::new()
            }
        }
    }

    pub async fn listar_gastos_filtrados_descompactados(
        &mut self,
        filtro: &FiltroGasto,
        limit: Option<u32>,
        offset: Option<u32>,
        sorter:&SortParameter
    ) -> Vec<serde_json::Value> {
        let (empresas, setores, caixas, pagamentos, fornecedores) = (
            self.listar_empresas().await,
            self.listar_setores().await,
            self.listar_caixas().await,
            self.listar_tipos_pagamento().await,
            self.listar_fornecedores().await,
        );

        let lista = self.listar_gastos_filtrados(filtro, limit, offset,sorter).await;
        lista
            .iter()
            .map(|gasto| {
                let setor = setores.iter().find(|v| v.id == gasto.id_setor).unwrap();
                serde_json::json!({
                "valor": gasto.valor,
                "nf": gasto.nf,
                "data": gasto.data,
                "setor": setor.nome,
                "empresa": empresas.iter().find(|v|v.id==setor.id_empresa).unwrap().nome,
                "caixa": caixas.iter().find(|v|v.id==gasto.id_caixa).unwrap().nome,
                "pagamento": pagamentos.iter().find(|v|v.id==gasto.id_tipo_pagamento).unwrap().nome,
                "fornecedor": fornecedores.iter().find(|v|v.id==gasto.id_fornecedor).unwrap().nome,
                "obs": gasto.obs,
                })
            })
            .collect()
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
        console::regular!("vendo se fornecedor ja existe");
        match self.obter_fornecedor(&nome).await {
            Some(f) => {
                //se info está diferente
                if f.id_setor != setor_preferido.id
                    || f.id_tipo_pagamento != pagamento_preferido.id
                    || f.id_caixa != caixa_preferido.id
                {
                    console::regular!(
                        "Fornecedor existe! Atualizando:\n{}",
                        &format!(
                            "UPDATE Fornecedores SET 
                                id_setor = {},
                                id_tipo_pagamento = {},
                                id_caixa = {},
                                modificado = date('now','localtime')
                            WHERE id = {};
                            ",
                            setor_preferido.id, pagamento_preferido.id, caixa_preferido.id, f.id
                        )
                    );
                    sqlx::query(&format!(
                        "UPDATE Fornecedores SET 
                                id_setor = {},
                                id_tipo_pagamento = {},
                                id_caixa = {},
                                modificado = date('now','localtime')
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
                console::regular!("Fornecedor nao existe");
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
                        console::bad!("e (line {}): {}", line!(), e);
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
    ) -> Result<Vec<String>, Vec<String>> {
        console::regular!("Registrar gasto()");
        let mut problemas = Vec::with_capacity(9);

        if self.obter_gasto(&fornecedor, nf).await.is_some() {
            problemas.push(String::from("NF já ultilizada para esse fornecedor"));
        }

        if problemas.len() == 0 {
            console::regular!("sem problemas até aqui");

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
                Ok(_) => Ok(Vec::new()),
                Err(e) => {
                    problemas.push(format!("Erro SQL: {}", e));
                    console::bad!("Erro SQL: {}", e);
                    Err(problemas)
                }
            }
        } else {
            Err(problemas)
        }
    }

    pub async fn registrar_gasto_de_json(
        &mut self,
        json_str: &str,
    ) -> Result<Vec<String>, Vec<String>> {
        console::regular!("Recebi json: {}", json_str);
        let mut problemas = Vec::with_capacity(9);

        if let Ok(json_obj) = serde_json::from_str::<serde_json::Value>(json_str) {
            let valor = match &json_obj["valor"] {
                serde_json::Value::Number(val) => Some(val.as_u64().unwrap() as u32),
                serde_json::Value::String(str_val) => {
                    match str_val.replace(".", "").replace(",", "").parse::<u32>() {
                        Ok(val) => Some(val),
                        Err(_) => {
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
                serde_json::Value::String(caixa_str) => {
                    if let Some(v) = self.obter_caixa(caixa_str).await {
                        Some(v)
                    } else {
                        problemas.push(String::from("caixa não encontrado"));
                        None
                    }
                }
                serde_json::Value::Null => {
                    console::alert!("é null");
                    problemas.push(String::from("nenhum caixa recebido"));
                    None
                }
                _ => {
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
                    Ok(v) => Some(v),
                    Err(_) => {
                        problemas.push(String::from("formato de data incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhuma data recebida"));
                None
            };

            let mut eh_novo_fornecedor = false;

            let fornecedor =
                if let serde_json::Value::String(fornecedor_str) = &json_obj["fornecedor"] {
                    if setor.is_some() && tipo_pagamento.is_some() && caixa.is_some() {
                        eh_novo_fornecedor = !self
                            .listar_fornecedores()
                            .await
                            .iter()
                            .find(|&x| &x.nome == fornecedor_str)
                            .is_some();

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
                                console::bad!("Erro ao reg/att fornecedor: {}", e);
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
                serde_json::Value::String(str_val) => match str_val.parse::<u32>() {
                    Ok(val) => Some(val),
                    Err(_) => {
                        problemas.push(String::from("nf mal-formatada"));
                        None
                    }
                },
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
                return match self
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
                    .await
                {
                    Ok(mut vetor) => {
                        if eh_novo_fornecedor {
                            vetor.push("Novo fornecedor adicionado".to_string());
                        }
                        return Ok(vetor);
                    }
                    Err(e) => Err(e),
                };
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
                        console::alert!("talvez seria bom alguma verificação de data...?");
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
                        problemas.push(String::from("formato da NF incorreto"));
                        None
                    }
                }
            } else {
                problemas.push(String::from("nenhuma NF recebida"));
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

    pub async fn importar_csv_aldeia(
        &mut self,
        fornecedores_path: &PathBuf,
        gastos_path: &PathBuf,
    ) -> Result<(), String> {
        console::regular!("Importando CSVs");
        let fornecedores = std::fs::read_to_string(fornecedores_path).unwrap();
        let gastos = std::fs::read_to_string(gastos_path).unwrap();

        let mut caixas: Vec<String> = Vec::with_capacity(20);
        let mut empresas: Vec<String> = Vec::with_capacity(20);
        let mut setores: Vec<(String, String)> = Vec::with_capacity(20);
        let mut pagamentos: Vec<String> = Vec::with_capacity(20);

        for linha in fornecedores.split('\n') {
            let record: Vec<String> = linha.split(',').map(trim_whitespace).collect();
            if record.len() == 4 {
                let empresa_setor: Vec<String> = record[1].split('_').map(|v|{trim_whitespace(v)}).collect();

                if !empresas.iter().find(|&x| x == &empresa_setor[0]).is_some() {
                    empresas.push(empresa_setor[0].clone());
                }
                if !setores
                    .iter()
                    .find(|&x| x.0 == empresa_setor[1] && x.1 == empresa_setor[0])
                    .is_some()
                {
                    setores.push((empresa_setor[1].clone(), empresa_setor[0].clone()));
                }

                if !caixas.iter().find(|&x| x == &record[3]).is_some() {
                    caixas.push(record[3].clone());
                }
                if !pagamentos.iter().find(|&x| x == &record[2]).is_some() {
                    pagamentos.push(record[2].clone());
                }
            } else {
                console::alert!("Não fez a linha: \"{}\"", linha);
            }
        }

        for linha in gastos.split('\n') {
            let record: Vec<String> = linha.split(',').map(|v|{trim_whitespace(v)}).collect();

            if record.len() == 8 {
                let empresa_setor: Vec<String> = record[1].split('_').map(|v|{trim_whitespace(v)}).collect();
                if !empresas.iter().find(|&x| x == &empresa_setor[0]).is_some() {
                    empresas.push(empresa_setor[0].clone());
                }
                if !setores
                    .iter()
                    .find(|&x| x.0 == empresa_setor[1] && x.1 == empresa_setor[0])
                    .is_some()
                {
                    setores.push((empresa_setor[1].clone(), empresa_setor[0].clone()));
                }
                if !caixas.iter().find(|&x| x == &record[6]).is_some() {
                    caixas.push(record[6].clone());
                }
                if !pagamentos.iter().find(|&x| x == &record[4]).is_some() {
                    pagamentos.push(record[4].clone());
                }
            } else {
                console::alert!("Não fez a linha: \"{}\"", linha);
            }
        }

        console::regular!(
            "Registrando: {:#?}",
            (&caixas, &empresas, &setores, &pagamentos)
        );

        let c:(Vec<&str>,Vec<&str>,Vec<(&str,&str)>,Vec<&str>) = (caixas.iter().map(|s| &**s).collect(),
        empresas.iter().map(|s| &**s).collect(),
        setores.iter().map(|s| ((&s.0 as &str), (&s.1 as &str))).collect(),
        pagamentos.iter().map(|s| &**s).collect());
        self.registrar_basicos(&c.0,&c.1,&c.2,&c.3)
            .await;

        for linha in fornecedores.split('\n') {
            let record: Vec<String> = linha.split(',').map(trim_whitespace).collect();

            if record.len() == 4 {
                let empresa_setor: Vec<String> = record[1].split('_').map(trim_whitespace).collect();

                let mut sucesso = false;

                if let Some(setor) = self.obter_setor(&empresa_setor[1], &empresa_setor[0]).await {
                    if let Some(pagamento) = self.obter_tipo_pagamento(&record[2]).await {
                        if let Some(caixa) = self.obter_caixa(&record[3]).await {
                            self.registrar_ou_atualizar_fornecedor(
                                &record[0], &setor, &pagamento, &caixa,
                            )
                            .await?;
                            sucesso = true;
                        }
                    }
                }
                if !sucesso {
                    return Err(format!("Erro com fornecedor: {:?}", record));
                }
            } else {
                console::alert!("Não fez a linha: \"{}\"", linha);
            }
        }
        for linha in gastos.split('\n') {
            let record: Vec<String> = linha.split(',').map(trim_whitespace).collect();
            if record.len() == 8 {
                let empresa_setor: Vec<&str> = record[1].split('_').collect();

                let valor = record[3]
                    .parse::<f64>()
                    .expect(&format!("Erro no valor {}", record[3]));
                let nf = record[5]
                    .parse::<u32>()
                    .expect(&format!("Erro na nf {}", record[5]));
                let mut data = SqlDateTime::parse_from_str(&record[2], "%d/%m/%Y")
                    .expect(&format!("Erro na data {}", record[2]));
                //datas no CSV podem estar com só dois digitos pro ano
                if data.year() < 2000 {
                    data = data
                        .with_year(data.year() + 2000)
                        .expect("Erro ao modificar data");
                }
                let setor = self
                    .obter_setor(empresa_setor[1], empresa_setor[0])
                    .await
                    .expect(&format!(
                        "Erro no setor {:?}",
                        (empresa_setor[1], empresa_setor[0])
                    ));
                let caixa = self
                    .obter_caixa(&record[6])
                    .await
                    .expect(&format!("Erro n caixa {}", record[6]));
                let pagamento = self
                    .obter_tipo_pagamento(&record[4])
                    .await
                    .expect(&format!("Erro no pagamento {}", record[4]));
                let fornecedor = match self.obter_fornecedor(&record[0]).await {
                    Some(f) => f,
                    None => self
                        .registrar_ou_atualizar_fornecedor(&record[0], &setor, &pagamento, &caixa)
                        .await
                        .expect(&format!(
                            "Erro ao registrar fornecedor desconhecido: {}",
                            record[0]
                        )),
                };

                self.registrar_gasto(
                    (valor * 100.0) as u32,
                    nf,
                    data,
                    &setor,
                    &caixa,
                    &pagamento,
                    &fornecedor,
                    record[7].to_owned(),
                )
                .await
                .expect("Erro ao registrar gasto");
            } else {
                console::alert!("Não fez a linha: \"{}\"", linha);
            }
        }

        Ok(())
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
                .expect(&format!(
                    "Tentando adicionar setor a empresa inexistente ({})",
                    empresa
                ))
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
                console::bad!("line {}: {}", line!(), e);
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
                    modificado = date('now','localtime')
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

    pub async fn renomear_fornecedor(&mut self, original: &str, novo:&str){
        let original = self.obter_fornecedor(original).await.expect("Erro ao ler fornecedor");
        if let Some(destino) = self.obter_fornecedor(novo).await{
            self.remover_fornecedor(&original, &destino).await;
        }
        else {
            sqlx::query(&format!(
                "UPDATE Fornecedores SET 
                        nome = '{}',
                        modificado = date('now','localtime')
                    WHERE id = {};
                    ",
                novo, original.id
            ))
            .execute(&self.0)
            .await
            .unwrap();
        }
    }
}

impl Drop for BancoDeDados {
    fn drop(&mut self) {
        executor::block_on(self.0.close());
    }
}
