interface Caixa {
	nome: string;
	modificado: string;
}
interface Empresa {
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
	return [
		{ nome: 'Santander', modificado: '2023-09-13' },
		{ nome: 'Bradesco', modificado: '2023-09-13' }
	];
}
export async function listarEmpresas(): Promise<Empresa[]> {
	return [
		{ nome: 'Hotel', modificado: '2023-09-13' },
		{ nome: 'Restaurante', modificado: '2023-09-13' }
	];
}
export async function listarSetores(): Promise<Setor[]> {
	return [
		{ nome:'Manutenção', empresa: 'Hotel', modificado: '2023-09-13' },
		{ nome:'Equipamento', empresa: 'Restaurante', modificado: '2023-09-13' }
	];
}
export async function listarFornecedores(): Promise<Fornecedor[]> {
	return [
		{
			nome: 'Maria',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria1',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria2',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria3',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria4',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria5',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria6',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria7',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria8',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria9',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria10',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'Maria11',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		},
		{
			nome: 'João',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Restaurante',
			pagamento: 'Cartão',
			caixa: 'Bradesco'
		}
	];
}
export class FiltroGastos {
	dataInicial = { v: '', h: false };
	dataFinal = { v: '', h: false };
	fornecedor = { v: '', h: false };
	empresa = { v: [''], h: false };
	setor = { v: [''], h: false };
	pagamento = { v: [''], h: false };
	caixa = { v: [''], h: false };

	equals(outro: FiltroGastos) {
		return JSON.stringify(this) == JSON.stringify(outro);
	}
}

export async function listarGastos(filtro:FiltroGastos, sortParameter:{v:string, d:boolean}): Promise<Gasto[]> {
	let todos = [
		{
			valor: 123,
			nf: 4638,
			data: '2022-09-13',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Hotel',
			pagamento: 'Cartão',
			caixa: 'Bradesco',
			fornecedor: 'Maria',
			obs: 'Muito a falar'
		},
		{
			valor: 22,
			nf: 223406,
			data: '2021-09-13',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Restaurante',
			pagamento: 'Dinheiro',
			caixa: 'Santander',
			fornecedor: 'João',
			obs: ''
		}
	];
	const repeat = (arr:any[], n:number) => [].concat(...Array(n).fill(arr));

	for (const key in filtro) {
		let value = (filtro as any)[key];
		if(value.h){
			todos = todos.filter((e)=>{
				return (e as any)[key] == value.v;
			});
		}
	}
	let gastos = repeat(todos, 50);

	//ordena de acordo com o sortParameter
	let parametro = sortParameter.v.toLowerCase();
	let sorter = (a: any, b: any) =>
		a[parametro].localeCompare(b[parametro], 'pt', { sensitivity: 'base' });
	if (parametro == 'data')
		sorter = (a: any, b: any) => new Date(a.data).getTime() - new Date(b.data).getTime();
	else if (parametro == 'nf' || parametro == 'valor') sorter = (a: any, b: any) => parseInt(a[parametro]) - parseInt(b[parametro]);
	else if (parametro.startsWith('obs')) {
		sorter = (a: any, b: any) => a.obs.localeCompare(b.obs, 'pt', { sensitivity: 'base' });
	}
	gastos.sort(sorter);
	if (sortParameter.d) gastos.reverse();
	return gastos;
}
export async function listarPagamentos(): Promise<TipoDePagamento[]> {
	return [
		{ nome: 'Cartão', modificado: '2023-09-13' },
		{ nome: 'Dinheiro', modificado: '2023-09-13' }
	];
}
