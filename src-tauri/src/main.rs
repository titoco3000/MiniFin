// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod storage;
pub mod tipos;

use futures::executor;
use std::path::PathBuf;
use std::sync::Mutex;
use storage::BancoDeDados;

pub type SqlDateTime = sqlx::types::chrono::NaiveDate;

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
fn listar_gastos(database: tauri::State<'_, Mutex<BancoDeDados>>, filtro: tipos::FiltroGasto) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap()
            .listar_gastos_filtrados(&filtro),
    ))
    .unwrap()
}

fn main() {
    println!("{:?}", storage::obter_local_padrao());
    let database = Mutex::new(match executor::block_on(storage::BancoDeDados::abrir()) {
        Ok(db) => db,
        Err(_) => {
            executor::block_on(storage::criar_database(&PathBuf::from(
                "/home/tito/Documents/raja.db",
            )))
            .unwrap();
            executor::block_on(storage::BancoDeDados::abrir()).unwrap()
        }
    });

    if executor::block_on(database.lock().unwrap().listar_caixas()).is_empty() {
        executor::block_on(database.lock().unwrap().registrar_basicos(
            &["Santander", "Bradesco"],
            &["Hotel", "Restaurante"],
            &[
                ("Manutenção", "Hotel"),
                ("Manutenção", "Restaurante"),
                ("Lavanderia", "Hotel"),
                ("Comida", "Restaurante"),
            ],
            &["Cartão", "Dinheiro"],
        ))
    }

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            listar_caixas,
            listar_tipos_pagamento,
            listar_setores,
            listar_empresas,
            listar_fornecedores,
            listar_gastos,
            validar_gasto,
            registrar_gasto
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
