<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputEmpresa from './InputEmpresa.svelte';
	import InputTipoPagamento from './InputTipoPagamento.svelte';
	import { listarGastos, type Gasto, FiltroGastos as Filtro } from '$lib/armazenamento';
	import { onMount } from 'svelte';
	import { obterDigitosNF } from '$lib/utils';

	const titulos = [
		'Data',
		'Fornecedor',
		'Empresa',
		'Setor',
		'Valor',
		'Pagamento',
		'NF',
		'Caixa',
		'Observações'
	];

	let filtroAplicado = new Filtro();
	let filtroAtual = new Filtro();
	let valoresReais = new Filtro();

	let valorToggle = {
		data_inicial: false,
		data_final: false,
		fornecedor: false,
		empresa: false,
		setor: false,
		tipo_pagamento: false,
		caixa: false
	}

	let filtroEl: HTMLElement;
	let setorToggleEl: HTMLInputElement;
	let empresaToggleEl: HTMLInputElement;
	let tableHeaderEl: HTMLElement;

	let dataInicialEl: HTMLInputElement;
	let dataFinalEl: HTMLInputElement;
	let fornecedorEl: { reset: Function };
	let empresaEl: { reset: Function };
	let setorEl: { reset: Function };
	let pagamentoEl: { reset: Function };
	let caixaEl: { reset: Function };

	let gastosFiltrados: Gasto[] = [];

	// direção: 0 para menor ao maior
	let sortParameter = { v: titulos[0], d: false };

	let somatorioValor = 0;
	let digitosNF = 9;

	function selecionarFiltro(e: any) {
		algoModificado();
		let el = e.srcElement.parentNode.querySelector('.input-holder');
		if (e.currentTarget.checked) {
			el.style.setProperty('--opacity', '0');
			el.style.setProperty('--blur-amount', '0px');
			el.style.setProperty('--pointer-events', 'none');
		} else {
			el.style.setProperty('--opacity', '0.5');
			el.style.setProperty('--blur-amount', '1px');
			el.style.setProperty('--pointer-events', 'all');
		}
	}

	function algoModificado() {
		//espera 1ms antes de agir para dar tempo de mudanças fazerem efeitos
		setTimeout(() => {
			
			//copia valores relevantes de valoresReais para filtroAtual
			for (const [key, value] of Object.entries(valorToggle)) {
				if(value)
					// @ts-ignore
					filtroAtual[key] = valoresReais[key];
				else
					// @ts-ignore
					filtroAtual[key] = [''];
			}
			if (!filtroAplicado.equals(filtroAtual)) {
				// console.log('diferentes');
			}
		}, 0);
	}

	function reset() {
		console.log('resetting');
		
		valoresReais.data_inicial[0] = new Date().toISOString().split('T')[0];
		valoresReais.data_final[0] = new Date().toISOString().split('T')[0];
		fornecedorEl.reset();
		empresaEl.reset();
		setorEl.reset();
		pagamentoEl.reset();
		caixaEl.reset();
		setTimeout(() => {
			console.log('valoresReais',valoresReais);
			
			filtroEl.querySelectorAll<HTMLInputElement>("input[type='checkbox']").forEach((e) => {
				if (e.checked) {
					e.click();
				}
			});
			carregarGastosComNovoFiltro();
		}, 1);
	}

	function carregarGastosComNovoFiltro() {
		//copia filtro
		filtroAplicado = Object.assign(new Filtro(), JSON.parse(JSON.stringify(filtroAtual)));
		carregarGastos();
	}
	function carregarGastos() {
		console.log(filtroAplicado);
		
		listarGastos(filtroAplicado, sortParameter).then((gastos) => {
			gastosFiltrados = gastos;
			somatorioValor = gastosFiltrados.reduce((a, b) => {
				return a + b.valor;
			}, 0);
		});
	}
	function setorModificado(v: string[]) {
		if (filtroAtual.empresa.length>0 && filtroAtual.empresa[0] != v[0]) {
			setTimeout(() => empresaToggleEl.click(), 1);
		}
		algoModificado();
	}
	function empresaModificada(v: string[]) {
		if (filtroAtual.setor.length>0 && filtroAtual.setor[0] != v[0]) {
			setTimeout(() => setorToggleEl.click(), 1);
		}
		algoModificado();
	}
	function headerButtonClick(titulo: string) {
		if (sortParameter.v == titulo) sortParameter.d = !sortParameter.d;
		else sortParameter.v = titulo;
		carregarGastos();
		titulos.forEach((value, i) => {
			let el = (tableHeaderEl.childNodes[i] as Element).querySelector('span');
			if (el) {
				if (value == sortParameter.v) {
					el.style.display = 'block';
					if (sortParameter.d) el.style.rotate = '0deg';
					else el.style.rotate = '180deg';
				} else el.style.display = 'none';
			}
		});
	}
	function formatarValor(v:number){
		let s:string = `${v}`;
		while(s.length<3) s = '0'+s;
		return s.slice(0, s.length-2) + "," + s.slice(s.length-2);
	}
	function formatarNF(nf:number){
		let s:string = `${nf}`;
		while(s.length<digitosNF) s = '0'+s;
		return s;
	}
	onMount(() => {
		digitosNF = obterDigitosNF();
		reset();
	});
</script>

<main>
	<div class="filtro" bind:this={filtroEl}>
		<h2>Filtros</h2>
		<div>
			<h3>Data inicial</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<input
						type="date"
						on:input={algoModificado}
						bind:value={valoresReais.data_inicial}
						bind:this={dataInicialEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={valorToggle.data_inicial}
				/>
			</div>
		</div>
		<div>
			<h3>Data final</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<input
						type="date"
						on:input={algoModificado}
						bind:value={valoresReais.data_final}
						bind:this={dataFinalEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={valorToggle.data_final}
				/>
			</div>
		</div>
		<div>
			<h3>Fornecedor</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputFornecedor
						onEdit={algoModificado}
						permitirNovo={false}
						bind:valor={valoresReais.fornecedor[0]}
						bind:this={fornecedorEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={valorToggle.fornecedor}
				/>
			</div>
		</div>
		<div>
			<h3>Empresa</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputEmpresa
						onEdit={empresaModificada}
						bind:valor={valoresReais.empresa}
						bind:this={empresaEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={(e) => {
						if (e.currentTarget.checked) {
							empresaModificada(valoresReais.empresa);
						}

						selecionarFiltro(e);
					}}
					bind:checked={valorToggle.empresa}
					bind:this={empresaToggleEl}
				/>
			</div>
		</div>
		<div>
			<h3>Setor</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputSetor
						onEdit={setorModificado}
						bind:valor={valoresReais.setor}
						bind:this={setorEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={(e) => {
						if (e.currentTarget.checked) {
							setorModificado(valoresReais.setor);
						}
						selecionarFiltro(e);
					}}
					bind:checked={valorToggle.setor}
					bind:this={setorToggleEl}
				/>
			</div>
		</div>
		<div>
			<h3>Tipo de pagamento</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputTipoPagamento
						onEdit={algoModificado}
						bind:valor={valoresReais.tipo_pagamento}
						bind:this={pagamentoEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={valorToggle.tipo_pagamento}
				/>
			</div>
		</div>
		<div>
			<h3>Caixa de entrada</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputCaixa
						onEdit={algoModificado}
						bind:valor={valoresReais.caixa}
						bind:this={caixaEl}
					/>
				</div>
				<input type="checkbox" on:change={selecionarFiltro} bind:checked={valorToggle.caixa} />
			</div>
		</div>
		<button type="button" on:click={reset}>Remover Filtros</button>
		<button type="button" on:click={carregarGastosComNovoFiltro}>Buscar</button>
	</div>
	<div class="table-holder">
		<table>
			<tr bind:this={tableHeaderEl}>
				{#each titulos as titulo}
					<th>
						<button
							class="sort-button"
							on:click={() => {
								headerButtonClick(titulo);
							}}
						>
							{titulo}<span class="arrow">^</span>
						</button>
					</th>
				{/each}
			</tr>
			{#each gastosFiltrados as gasto}
				<tr>
					<td>{new Date(gasto.data).toLocaleDateString('pt-br')}</td>
					<td>{gasto.fornecedor}</td>
					<td>{gasto.empresa}</td>
					<td>{gasto.setor}</td>
					<td>{formatarValor(gasto.valor)}</td>
					<td>{gasto.pagamento}</td>
					<td>{ formatarNF(gasto.nf)}</td>
					<td>{gasto.caixa}</td>
					<td>{gasto.obs}</td>
				</tr>
			{/each}
			<tfoot>
				<tr>
					<td colspan="4">Soma</td>
					<td>{formatarValor(somatorioValor)}</td>
					<td colspan="4"></td>
				</tr>
			</tfoot>
		</table>
	</div>
</main>

<style>
	main {
		display: flex;
		width: 100%;
		height: 100%;
	}
	.filtro {
		width: 250px;
		background-color: white;
		border: 2px solid black;
		background-color: var(--cor-tema-fraca);
		display: block;
		resize: both;
	}
	.filtro h2{
		background-color: var(--cor-tema-forte);
		margin: 0;
		padding: 5px;
		font-weight: 100;
		font-size: 22px;
	}
	.filtro h3 {
		margin: 0;
		flex-grow: 1;
		width: 100%;
		font-weight: 600;
	}
	.filtro > div {
		display: flex;
		flex-direction: column;
		border-bottom: 4px solid white;
		width: 100%;
		padding: 10px;
	}
	.controls-holder {
		display: flex;
		justify-content: space-between;
		width: 100%;
	}
	.input-holder {
		flex-grow: 1;
		flex-shrink: 1;
		background-color: aquamarine;
		position: relative;
		padding: 0px;
		width: 100%;
		height: 40px;
		margin: 0;
		min-width: 0;
		--blur-amount: 1px;
		--opacity: 0.5;
		--pointer-events: all;
	}
	input[type="checkbox"]{
		margin-left: 10px;
	}
	.input-holder::after {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		pointer-events: var(--pointer-events);
		border-radius: var(--tema-border-radius);
		/* slightly transparent fallback */
		background-color: rgba(255, 255, 255, .9);
	}
	
	/* if backdrop support: very transparent and blurred */
	@supports ((-webkit-backdrop-filter: none) or (backdrop-filter: none)) {
		.input-holder::after {
			background-color: rgba(255, 255, 255, var(--opacity));
			backdrop-filter: blur(var(--blur-amount));
			-webkit-backdrop-filter: blur(var(--blur-amount));
		}
	}

	.filtro button{
		background-color: var(--cor-tema-forte);
		border-radius: var(--tema-border-radius);
		border: none;
	}

	.table-holder {
		flex-grow: 1;
		overflow: auto;
	}
	.sort-button {
		height: 100%;
		font: inherit;
		padding: 0.25rem;
		width: 100%;
		border: 0;
		background-color: transparent;
		display: flex;
		justify-content: space-between;
	}
	.sort-button span {
		display: none;
	}
	th:first-child .sort-button span {
		display: block;
		rotate: 180deg;
	}
	th {
		background-color: var(--cor-tema-fraca);
		padding: 0;
	}

	table {
		table-layout: fixed;
		width: 100%;
		text-align: left;
		position: relative;
		border-collapse: separate;
		border-spacing: 0;
	}
	th,
	td {
		white-space: nowrap;
		overflow: hidden;
	}
	th {
		position: sticky;
		top: 0; /* Don't forget this, required for the stickiness */
		border-top: 2px solid;
		border-bottom: 2px solid;
		border-right: 2px solid;
	}
	tfoot td {
		position: sticky;
		bottom: 0; /* Don't forget this, required for the stickiness */
		border-top: 2px solid;
		border-bottom: 2px solid;
		border-right: 2px solid;
		background-color: var(--cor-tema-fraca);
	}
	td {
		background-color: white;
		border-bottom: 1px solid;
		border-right: 1px solid;
		padding: 0.25rem;
	}
	table th:first-child,
	table td:first-child {
		/* Apply a left border on the first <td> or <th> in a row */
		border-left: 2px solid;
	}

	th:nth-child(2) {
		width: 13%;
	}
	th:nth-child(6) {
		width: 13%;
	}
	th:nth-child(9) {
		width: 15%;
	}
</style>
