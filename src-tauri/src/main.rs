// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;

pub mod storage;
pub mod tipos;
#[macro_use]
pub mod custom_console;

use custom_console::macros::{self as console, extract};
use futures::executor;
use std::{
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};
use storage::BancoDeDados;
use tipos::SortParameter;

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
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .listar_tipos_pagamento(),
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
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .listar_fornecedores(),
    ))
    .unwrap()
}

#[tauri::command]
fn validar_gasto(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    json_data: &str,
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .validar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn registrar_gasto(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    json_data: &str,
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .registrar_gasto_de_json(json_data),
    ))
    .unwrap()
}

#[tauri::command]
fn listar_gastos(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    filtro: tipos::FiltroGasto,
    limit: u32,
    offset: u32,
    sorter: SortParameter,
) -> String {
    serde_json::to_string(&executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
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
fn definir_local_bd(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    local: &str,
) -> String {
    let mut local = PathBuf::from(local);
    local.push("minifin");
    local.push("minifin.db");
    serde_json::to_string(&match storage::Config::new(local.clone()).salvar() {
        Ok(_) => {
            let db: Option<BancoDeDados> = match executor::block_on(storage::BancoDeDados::abrir())
            {
                Ok(db) => Some(db),
                Err(_) => match executor::block_on(storage::criar_database(&local)) {
                    Ok(_) => match executor::block_on(storage::BancoDeDados::abrir()) {
                        Ok(bd) => Some(bd),
                        Err(e) => {
                            console::bad!("Erro ao abrir bd depos de criar: {}", e);
                            None
                        }
                    },
                    Err(e) => {
                        console::bad!("Erro ao criar banco de dados: {}", e);
                        None
                    }
                },
            };

            match db {
                Some(db) => {
                    *database.lock().unwrap() = Some(db);
                    Ok(())
                }
                None => Err("Erro ao abrir bd depos de criar".to_owned()),
            }
        }
        Err(_) => {
            console::regular!("Erro ao salvar config");
            Err("Erro ao salvar config".to_owned())
        }
    })
    .expect("Erro ao jsonificar")
}

#[tauri::command]
fn contar_gastos(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    filtro: tipos::FiltroGasto,
) -> u32 {
    executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .contar_gastos(&filtro),
    )
}
#[tauri::command]
fn somar_gastos(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    filtro: tipos::FiltroGasto,
) -> u32 {
    executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .somar_gastos(&filtro),
    )
}

#[tauri::command]
fn remover_gasto(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    fornecedor: String,
    nf: u32,
) {
    executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .remover_gasto(nf, &fornecedor),
    )
    .unwrap();
}

#[tauri::command]
fn renomear_fornecedor(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    original: String,
    novo: String,
) {
    executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .renomear_fornecedor(&original, &novo),
    );
}

#[tauri::command]
async fn extrair_dados_terminal() -> Vec<(String, String)> {
    use custom_console::buffer_queue::LogMessage::*;
    extract!()
        .iter()
        .map(|d| match d {
            Regular(m) => (m.clone(), "Regular".to_owned()),
            Good(m) => (m.clone(), "Good".to_owned()),
            Alert(m) => (m.clone(), "Alert".to_owned()),
            Bad(m) => (m.clone(), "Bad".to_owned()),
        })
        .collect()
}

fn trim_first_and_last(mut s: String) -> String {
    s.pop(); // remove last
    if s.len() > 0 {
        s.remove(0); // remove first
    }
    s
}
fn substr(s: &str, begin: usize, end: Option<usize>) -> Option<&str> {
    use std::iter::once;
    let mut itr = s.char_indices().map(|(n, _)| n).chain(once(s.len()));
    let begin_byte = itr.nth(begin)?;
    let end_byte = match end {
        Some(end) if begin >= end => begin_byte,
        Some(end) => itr.nth(end - begin - 1)?,
        None => s.len(),
    };
    Some(&s[begin_byte..end_byte])
}
async fn save_sheet(mut workbook: rust_xlsxwriter::Workbook, mut path: PathBuf) {
    use regex::Regex;
    let has_trailing_number = Regex::new(r".*\(\d*\)\.xlsx$").unwrap();
    let mut tentativa = 0;
    loop {
        match workbook.save(&path) {
            Ok(_) => {
                match open::that(&path) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        println!("Erro: {}", e);
                    }
                }
            },
            Err(_) => {},
        };
        let path_string = &path.display().to_string();
        if has_trailing_number.is_match(&path_string) {
            let before = substr(
                substr(&path_string, 0, Some(path_string.rfind(')').unwrap())).unwrap(),
                0,
                Some(path_string.rfind('(').unwrap())
            )
            .unwrap();
            path = PathBuf::from_str(&format!("{}({}).xlsx", before, tentativa)).unwrap();
        }
        else{
            let mut filename = path.with_extension("").file_name().unwrap().to_owned();
            filename.push(std::ffi::OsStr::new("(1).xlsx"));
            path.pop();
            path.push(filename);
        }
        tentativa+=1;
    }
}

#[tauri::command]
fn exportar_para_xlsx(
    database: tauri::State<'_, Mutex<Option<BancoDeDados>>>,
    filtro: tipos::FiltroGasto,
    sorter: SortParameter,
) -> Result<(), String> {
    use rust_xlsxwriter::*;

    let mut workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    let formatacao_header = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_background_color("#ffeeb2");
    let formatacao_corpo = Format::new().set_border(FormatBorder::Thin);
    let formatacao_nf = formatacao_corpo.clone().set_num_format("000000000");
    let formatacao_valor = formatacao_corpo.clone().set_num_format("#,##0.00");
    let formatacao_data = formatacao_corpo.clone().set_num_format("dd/mm/yyyy");

    // Set the column width for clarity.
    for (i, (title, width)) in [
        ("Data", 12),
        ("Fornecedor", 20),
        ("Empresa", 17),
        ("Setor", 18),
        ("Valor", 12),
        ("NF", 15),
        ("Caixa", 17),
        ("Observações", 50),
    ]
    .iter()
    .enumerate()
    {
        worksheet
            .set_column_width(i.try_into().unwrap(), *width)
            .unwrap();
        worksheet
            .write_with_format(0, i.try_into().unwrap(), *title, &formatacao_header)
            .unwrap();
    }

    let lista = executor::block_on(
        database
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .listar_gastos_filtrados_descompactados(&filtro, None, None, &sorter),
    );
    let tamanho = lista.len() as u32;
    for (i, linha) in lista.iter().enumerate() {
        worksheet
            .write_with_format(
                i as u32 + 1,
                0,
                ExcelDateTime::parse_from_str(&trim_first_and_last(linha["data"].to_string()))
                    .unwrap(),
                &formatacao_data,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                1,
                trim_first_and_last(linha["fornecedor"].to_string()),
                &formatacao_corpo,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                2,
                trim_first_and_last(linha["empresa"].to_string()),
                &formatacao_corpo,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                3,
                trim_first_and_last(linha["setor"].to_string()),
                &formatacao_corpo,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                4,
                linha["valor"].to_string().parse::<f64>().unwrap() / 100.0,
                &formatacao_valor,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                5,
                linha["nf"].to_string().parse::<u32>().unwrap(),
                &formatacao_nf,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                6,
                trim_first_and_last(linha["caixa"].to_string()),
                &formatacao_corpo,
            )
            .unwrap();
        worksheet
            .write_with_format(
                i as u32 + 1,
                7,
                trim_first_and_last(linha["obs"].to_string()),
                &formatacao_corpo,
            )
            .unwrap();
    }
    
    //coloca footer
    worksheet.write_with_format(tamanho+1, 0, "", &formatacao_header).unwrap();
    worksheet.write_with_format(tamanho+1, 1, "", &formatacao_header).unwrap();
    worksheet.write_with_format(tamanho+1, 2, "", &formatacao_header).unwrap();
    worksheet.write_with_format(tamanho+1, 3, "Total", &formatacao_header).unwrap();
    //worksheet.write_with_format(tamanho+1, 4, valor_total, &&formatacao_valor.clone().set_background_color("#ffeeb2")).unwrap();
    worksheet.write_formula_with_format(tamanho+1, 4, &format!("=SUM(E1:E{})",tamanho+1) as &str , &&formatacao_valor.clone().set_background_color("#ffeeb2")).unwrap();
    worksheet.write_with_format(tamanho+1, 5, "", &formatacao_header).unwrap();
    worksheet.write_with_format(tamanho+1, 6, "", &formatacao_header).unwrap();
    worksheet.write_with_format(tamanho+1, 7, "", &formatacao_header).unwrap();


    
    let pool = futures::executor::ThreadPool::new().expect("Failed to build pool");
    let mut path = dirs_2::download_dir().unwrap();
    path.push("relatorio.xlsx");
    pool.spawn_ok(save_sheet(workbook, path));
    
    Ok(())
}

fn main() {
    let db_mutex = Mutex::new(match storage::Config::ler() {
        Ok(_config_file) => match executor::block_on(storage::BancoDeDados::abrir()) {
            Ok(database) => Some(database),
            Err(_) => {
                console::alert!("Erro ao abrir banco de dados");
                None
            }
        },
        Err(_) => None,
    });

    let eh_instalacao = db_mutex.lock().unwrap().is_some();

    tauri::Builder::default()
        .manage(db_mutex)
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                use window_shadows::set_shadow;
                set_shadow(&window, true).expect("Unsupported platform!");
            }

            if eh_instalacao {
                window
                    .eval("window.location.replace('form');")
                    .expect("erro ao executar js");
            }
            Ok(())
        })
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
            definir_local_bd,
            contar_gastos,
            somar_gastos,
            remover_gasto,
            renomear_fornecedor,
            extrair_dados_terminal,
            exportar_para_xlsx
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
