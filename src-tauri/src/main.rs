// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod storage;
pub mod tipos;

use futures::executor;
use std::{path::PathBuf, sync::Mutex};
use storage::BancoDeDados;

pub type SqlDateTime = sqlx::types::chrono::NaiveDate;

// Tauri commands
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn listar_caixas(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().listar_caixas(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_tipos_pagamento(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().listar_tipos_pagamento(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_setores(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().listar_setores(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_empresas(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().listar_empresas(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_fornecedores(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().listar_fornecedores(),
    ))
    .unwrap()
}

#[tauri::command]
fn validar_gasto(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>, json_data: &str) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().validar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn registrar_gasto(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>, json_data: &str) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().as_mut().unwrap().registrar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_gastos(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    filtro: tipos::FiltroGasto,
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap()
            .listar_gastos_filtrados_descompactados(&filtro),
    ))
    .unwrap()
}

#[tauri::command]
async fn importar_csv_aldeia(fornecedores: String, gastos: String) -> String {
    serde_json::to_string(&executor::block_on(
        executor::block_on(storage::BancoDeDados::abrir())
            .unwrap()
            .importar_csv_aldeia(&PathBuf::from(fornecedores), &PathBuf::from(gastos)),
    ))
    .unwrap()
}
#[tauri::command]
fn checar_tipo_de_janela(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>) -> String {
    if database.lock().unwrap().is_none() {
        "instalacao".to_owned()
    } else {
        "normal".to_owned()
    }
}

#[tauri::command]
fn definir_local_bd(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>, local: &str) -> String {
    println!("definir_local_bd");
    let mut local = PathBuf::from(local);
    local.push("raja");
    local.push("raja.db");
    serde_json::to_string(&match storage::Config::new(local.clone()).salvar() {
         Ok(_) => {
            let db:Option<BancoDeDados> = match executor::block_on(storage::BancoDeDados::abrir()) {
                Ok(db)=>Some(db),
                Err(_)=>{
                    match executor::block_on(storage::criar_database(&local)) {
                        Ok(_)=>{
                            match executor::block_on(storage::BancoDeDados::abrir()) {
                                Ok(bd)=>Some(bd),
                                Err(e)=>{
                                    println!("Erro ao abrir bd depos de criar: {}", e);
                                    None
                                }
                            }
                        },
                        Err(e)=>{
                            println!("Erro ao criar bd: {}", e);
                            None
                        }
                    }
                }
            };

            match db {
                Some(db)=>{
                    *database.lock().unwrap() = Some(db);
                    Ok(())
                },
                None =>
                    Err("Erro ao abrir bd depos de criar".to_owned())
            }
         }
        Err(_) => {
            println!("Erro ao salvar config");
            Err("Erro ao salvar config".to_owned())

        }
    }).expect("Erro ao jsonificar")
}

fn main() {
    let db_mutex = Mutex::new( match storage::Config::ler() {
        Ok(_config_file) => match executor::block_on(storage::BancoDeDados::abrir()) {
            Ok(database) => {
                Some(database)
            }
            Err(e) => {
                panic!("Erro ao abrir db: {}", e);
            }
        },
        Err(_) => {
            None
        }
    });
    tauri::Builder::default()
    .manage(db_mutex)
    .invoke_handler(tauri::generate_handler![
        listar_caixas,
        listar_tipos_pagamento,
        listar_setores,
        listar_empresas,
        listar_fornecedores,
        listar_gastos,
        validar_gasto,
        registrar_gasto,
        importar_csv_aldeia,
        checar_tipo_de_janela,
        definir_local_bd
    ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}