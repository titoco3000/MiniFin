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
interface Gasto {
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
export async function listarGastos(): Promise<Gasto[]> {
	return [
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
			data: '2022-09-13',
			modificado: '2023-09-13',
			setor: 'manutenção',
			empresa: 'Restaurante',
			pagamento: 'Dinheiro',
			caixa: 'Santander',
			fornecedor: 'João',
			obs: ''
		}
	];
}
export async function listarPagamentos(): Promise<TipoDePagamento[]> {
	return [
		{ nome: 'Cartão', modificado: '2023-09-13' },
		{ nome: 'Dinheiro', modificado: '2023-09-13' }
	];
}
