// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

pub mod storage;
pub mod tipos;
#[macro_use]
pub mod custom_console;

use futures::executor;
use tipos::SortParameter;
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
    limit:u32,
    offset:u32,
    sorter:SortParameter
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap()
            .listar_gastos_filtrados_descompactados(&filtro, Some(limit), Some(offset), &sorter),
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
    local.push("minifin");
    local.push("minifin.db");
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

#[tauri::command]
fn contar_gastos(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,filtro: tipos::FiltroGasto) -> u32{
    executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap().contar_gastos(&filtro))
}
#[tauri::command]
fn somar_gastos(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,filtro: tipos::FiltroGasto) -> u32{
    executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap().somar_gastos(&filtro))
}

#[tauri::command]
fn remover_gasto(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,fornecedor:String,nf:u32){
    executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap().remover_gasto(nf,&fornecedor)).unwrap();
}

#[tauri::command]
fn renomear_fornecedor(database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,original:String, novo:String){
    executor::block_on(
        database
            .lock()
            .unwrap().as_mut().unwrap().renomear_fornecedor(&original,&novo));
}

use custom_console::macros as console;
fn main() {
    console::teste();
    console::regular!("oie");
    console::good!("oie");
    console::alert!("oie");
    console::bad!("oie");
    // let db_mutex = Mutex::new( match storage::Config::ler() {
    //     Ok(_config_file) => match executor::block_on(storage::BancoDeDados::abrir()) {
    //         Ok(database) => {
    //             Some(database)
    //         }
    //         Err(e) => {
    //             panic!("Erro ao abrir db: {}", e);
    //         }
    //     },
    //     Err(_) => {
    //         None
    //     }
    // });

    // let eh_instalacao = db_mutex.lock().unwrap().is_some();

    // tauri::Builder::default()
    // .manage(db_mutex)
    // .setup(move |app| {
    //     let window = app.get_window("main").unwrap();
        
    //     #[cfg(target_os = "windows")]
    //     {
    //         use window_shadows::set_shadow;
    //         set_shadow(&window, true).expect("Unsupported platform!");
    //     }
        
    //     if eh_instalacao{
    //         window.eval("window.location.replace('form');").expect("erro ao executar js");
    //     }
    //     Ok(())
    // })
    // .invoke_handler(tauri::generate_handler![
    //     listar_caixas,
    //     listar_tipos_pagamento,
    //     listar_setores,
    //     listar_empresas,
    //     listar_fornecedores,
    //     listar_gastos,
    //     validar_gasto,
    //     registrar_gasto,
    //     importar_csv_aldeia,
    //     checar_tipo_de_janela,
    //     definir_local_bd,
    //     contar_gastos,
    //     somar_gastos,
    //     remover_gasto,
    //     renomear_fornecedor
    // ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}