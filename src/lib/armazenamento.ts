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

	equals(outro: FiltroGastos) {
		return JSON.stringify(this) == JSON.stringify(outro);
	}
}

export async function listarGastos(filtro:FiltroGastos, sortParameter:{v:string, d:boolean}): Promise<Gasto[]> {
	let jsonified = JSON.stringify(filtro);
	console.log("Pedindo com parametro:",jsonified);
	let v:string = await invoke('listar_gastos',{filtro: filtro});
	return JSON.parse(v);
	// let todos = [
	// 	{
	// 		valor: 123,
	// 		nf: 4638,
	// 		data: '2022-09-13',
	// 		modificado: '2023-09-13',
	// 		setor: 'manutenção',
	// 		empresa: 'Hotel',
	// 		pagamento: 'Cartão',
	// 		caixa: 'Bradesco',
	// 		fornecedor: 'Maria',
	// 		obs: 'Muito a falar'
	// 	},
	// 	{
	// 		valor: 22,
	// 		nf: 223406,
	// 		data: '2021-09-13',
	// 		modificado: '2023-09-13',
	// 		setor: 'manutenção',
	// 		empresa: 'Restaurante',
	// 		pagamento: 'Dinheiro',
	// 		caixa: 'Santander',
	// 		fornecedor: 'João',
	// 		obs: ''
	// 	}
	// ];
	// const repeat = (arr:any[], n:number) => [].concat(...Array(n).fill(arr));

	// for (const key in filtro) {
	// 	let value = (filtro as any)[key];
	// 	if(value.h){
	// 		todos = todos.filter((e)=>{
	// 			return (e as any)[key] == value.v;
	// 		});
	// 	}
	// }
	// let gastos = repeat(todos, 50);

	// //ordena de acordo com o sortParameter
	// let parametro = sortParameter.v.toLowerCase();
	// let sorter = (a: any, b: any) =>
	// 	a[parametro].localeCompare(b[parametro], 'pt', { sensitivity: 'base' });
	// if (parametro == 'data')
	// 	sorter = (a: any, b: any) => new Date(a.data).getTime() - new Date(b.data).getTime();
	// else if (parametro == 'nf' || parametro == 'valor') sorter = (a: any, b: any) => parseInt(a[parametro]) - parseInt(b[parametro]);
	// else if (parametro.startsWith('obs')) {
	// 	sorter = (a: any, b: any) => a.obs.localeCompare(b.obs, 'pt', { sensitivity: 'base' });
	// }
	// gastos.sort(sorter);
	// if (sortParameter.d) gastos.reverse();
	// return gastos;
}
export async function listarPagamentos(): Promise<TipoDePagamento[]> {
	let v:string = await invoke('listar_tipos_pagamento');
	return JSON.parse(v);
}

export async function enviarNovoGasto(gasto:Gasto) {
	let v:string = await invoke('registrar_gasto',{jsonData: JSON.stringify(gasto)});
	return JSON.parse(v);
}