// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod storage;
pub mod tipos;

use futures::executor;
use std::{path::PathBuf, sync::Mutex};
use storage::BancoDeDados;

pub type SqlDateTime = sqlx::types::chrono::NaiveDate;

static mut EH_INSTALACAO: bool = false;

// Tauri commands
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn listar_caixas(database: tauri::State<'_, Mutex<BancoDeDados>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().listar_caixas(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_tipos_pagamento(database: tauri::State<'_, Mutex<BancoDeDados>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().listar_tipos_pagamento(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_setores(database: tauri::State<'_, Mutex<BancoDeDados>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().listar_setores(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_empresas(database: tauri::State<'_, Mutex<BancoDeDados>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().listar_empresas(),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_fornecedores(database: tauri::State<'_, Mutex<BancoDeDados>>) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().listar_fornecedores(),
    ))
    .unwrap()
}

#[tauri::command]
fn validar_gasto(database: tauri::State<'_, Mutex<BancoDeDados>>, json_data: &str) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().validar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn registrar_gasto(database: tauri::State<'_, Mutex<BancoDeDados>>, json_data: &str) -> String {
    serde_json::to_string(&executor::block_on(
        database.lock().unwrap().registrar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_gastos(
    database: tauri::State<'_, Mutex<BancoDeDados>>,
    filtro: tipos::FiltroGasto,
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap()
            .listar_gastos_filtrados_descompactados(&filtro),
    ))
    .unwrap()
}

#[tauri::command]
async fn importar_csv_aldeia(fornecedores: String, gastos: String) {
    storage::BancoDeDados::abrir()
        .await
        .unwrap()
        .importar_csv_aldeia(&PathBuf::from(fornecedores), &PathBuf::from(gastos))
        .await
        .unwrap();
}
#[tauri::command]
fn checar_tipo_de_janela() -> String {
    if unsafe { EH_INSTALACAO } {
        "instalacao".to_owned()
    } else {
        "normal".to_owned()
    }
}

#[tauri::command]
fn definir_local_bd(local: &str) -> String {
    let mut local = PathBuf::from(local);
    local.push("raja");
    local.push("raja.db");
    match storage::Config::new(local.clone()).salvar() {
        Ok(_) => match executor::block_on(storage::BancoDeDados::abrir()) {
            Ok(_db) => {
                println!("bd já existe");
                "Ok"
            }
            Err(_) => match executor::block_on(storage::criar_database(&local)) {
                Ok(_db) => "Ok",
                Err(e) => {
                    println!("Erro ao criar bd: {}",e);
                    "Err"
                }
            },
        },
        Err(_) => {
            println!("Erro ao salvar config");
            "Err"
        }
    }
    .to_owned()
}

fn main() {
    println!("{:?}", storage::obter_local_padrao());

    let mut builder = tauri::Builder::default();

    match storage::Config::ler() {
        Ok(_config_file) => match executor::block_on(storage::BancoDeDados::abrir()) {
            Ok(database) => {
                let db_mutex = Mutex::new(database);

                builder = builder
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
                        checar_tipo_de_janela
                    ]);
            }
            Err(e) => {
                panic!("Erro ao abrir db: {}", e);
            }
        },
        Err(_) => {
            unsafe {
                EH_INSTALACAO = true;
            }

            builder = builder.invoke_handler(tauri::generate_handler![
                importar_csv_aldeia,
                checar_tipo_de_janela,
                definir_local_bd
            ]);
        }
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let database = Mutex::new(match executor::block_on(storage::BancoDeDados::abrir()) {
    //     Ok(db) => db,
    //     Err(_) => {
    //         executor::block_on(storage::criar_database(&PathBuf::from(
    //             "/home/tito/Documents/raja/raja.db",
    //         )))
    //         .unwrap();
    //         executor::block_on(storage::BancoDeDados::abrir()).unwrap()
    //     }
    // });

    // if executor::block_on(database.lock().unwrap().listar_caixas()).is_empty() {
    //     executor::block_on(database.lock().unwrap().registrar_basicos(
    //         &["Santander", "Bradesco"],
    //         &["Hotel", "Restaurante"],
    //         &[
    //             ("Manutenção", "Hotel"),
    //             ("Manutenção", "Restaurante"),
    //             ("Lavanderia", "Hotel"),
    //             ("Comida", "Restaurante"),
    //         ],
    //         &["Cartão", "Dinheiro"],
    //     ))
    // }
}
