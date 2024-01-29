import { invoke } from '@tauri-apps/api/tauri'

interface Caixa {
	nome: string;
	modificado: string;
}
interface Empresa {
	id:number;
	nome: string;
	modificado: string;
}
interface Setor {
	nome: string;
	empresa: string;
	modificado: string;
}
interface Fornecedor {
	nome: string;
	modificado: string;
	setor: string;
	empresa: string;
	pagamento: string;
	caixa: string;
}
export interface Gasto {
	valor: number;
	nf: number;
	data: string;
	modificado: string;
	setor: string;
	empresa: string;
	pagamento: string;
	caixa: string;
	fornecedor: string;
	obs: string;
}


interface TipoDePagamento {
	nome: string;
	modificado: string;
}

export async function listarCaixas(): Promise<Caixa[]> {
	let v:string = await invoke('listar_caixas');
	return JSON.parse(v);
}
export async function listarEmpresas(): Promise<Empresa[]> {
	let v:string = await invoke('listar_empresas');
	return JSON.parse(v);
}
export async function listarSetores(): Promise<Setor[]> {
	// return [
	// 	{ nome:'Manutenção', empresa: 'Hotel', modificado: '2023-09-13' },
	// 	{ nome:'Equipamento', empresa: 'Restaurante', modificado: '2023-09-13' }
	// ];
	let jsonData:string = await invoke('listar_setores');
	let setores = JSON.parse(jsonData);
	let empresas =  await listarEmpresas();
	
	for (let i = 0; i < setores.length; i++) {
		setores[i].empresa = empresas.find(obj => {
			return obj.id === setores[i].id_empresa
		  })?.nome;
		delete setores[i].id_empresa;
	} 
	
	return setores;
}
export async function listarFornecedores(): Promise<Fornecedor[]> {
	let v:string = await invoke('listar_fornecedores');
	return JSON.parse(v);
}

export class FiltroGastos {
	data_inicial:string[] = [''];
	data_final:string[] = [''];
	fornecedor:string[] = [''];
	empresa:string[] = [''];
	setor:string[] = [''];
	tipo_pagamento:string[] = [''];
	caixa:string[] = [''];
	obs_pesquisa:string[] = [''];
	conteudo:string[] = [''];

	equals(outro: FiltroGastos) {
		return JSON.stringify(this) == JSON.stringify(outro);
	}
}

export async function listarGastos(filtro:FiltroGastos, sortParameter:{i:number, d:boolean}, limit:number, offset:number): Promise<Gasto[]> {
	let jsonified = JSON.stringify(filtro);
	console.log("Pedindo com parametro:",jsonified);
	let v:string = await invoke('listar_gastos',{filtro: filtro,limit:limit, offset:offset, sorter:sortParameter});
	console.log("Recebi; ",JSON.parse(v));
	
	return JSON.parse(v);
}
export async function listarPagamentos(): Promise<TipoDePagamento[]> {
	let v:string = await invoke('listar_tipos_pagamento');
	return JSON.parse(v);
}

export async function enviarNovoGasto(gasto:Gasto) {
	let v:string = await invoke('registrar_gasto',{jsonData: JSON.stringify(gasto)});
	return JSON.parse(v);
}

export async function importarCSVs(fornecedores:string, gastos:string) {
	let v:string = await invoke('importar_csv_aldeia',{fornecedores: fornecedores, gastos:gastos});
	return JSON.parse(v);
}

export async function definirLocalDB(local:string) {
	let v:string = await invoke('definir_local_bd',{local:local});
	return JSON.parse(v);
}
export async function contarGastos(filtro:FiltroGastos):Promise<number> {
	return await invoke('contar_gastos',{filtro:filtro});
}
export async function somarGastos(filtro:FiltroGastos):Promise<number> {
	return await invoke('somar_gastos',{filtro:filtro});
}

export async function excluirGasto(fornecedor:string, nf:number) {
	invoke('remover_gasto',{fornecedor:fornecedor,nf:nf});
}