<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputEmpresa from './InputEmpresa.svelte';
	import InputTipoPagamento from './InputTipoPagamento.svelte';
	import { listarGastos, type Gasto, FiltroGastos as Filtro } from '$lib/armazenamento';
	import { onMount } from 'svelte';

	let filtroAplicado = new Filtro();
	let filtroAtual = new Filtro();

	let filtroEl: HTMLElement;
	let setorToggleEl: HTMLInputElement;
	let empresaToggleEl: HTMLInputElement;

	let dataInicialEl: HTMLInputElement;
	let dataFinalEl: HTMLInputElement;
	let fornecedorEl: { reset: Function };
	let empresaEl: { reset: Function };
	let setorEl: { reset: Function };
	let pagamentoEl: { reset: Function };
	let caixaEl: { reset: Function };

	let gastosFiltrados: Gasto[] = [];

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
			if (!filtroAplicado.equals(filtroAtual)) {
				// console.log('diferentes');
			}
		}, 1);
	}

	function reset() {
		dataInicialEl.value = '';
		dataFinalEl.value = '';
		fornecedorEl.reset();
		empresaEl.reset();
		setorEl.reset();
		pagamentoEl.reset();
		caixaEl.reset();
		setTimeout(() => {
			filtroEl.querySelectorAll<HTMLInputElement>("input[type='checkbox']").forEach((e) => {
				if (e.checked) {
					e.click();
				}
			});
		}, 1);
	}
	function carregarGastos() {
		//copia filtro
		filtroAplicado = Object.assign(new Filtro(), JSON.parse(JSON.stringify(filtroAtual)));
		listarGastos(filtroAplicado).then((gastos) => {
			gastosFiltrados = gastos;
		});
	}
	function setorModificado(v: string[]) {
		if (filtroAtual.empresa.h && filtroAtual.empresa.v[0] != v[0]) {
			setTimeout(() => empresaToggleEl.click(), 1);
		}
		algoModificado();
	}
	function empresaModificada(v: string[]) {
		if (filtroAtual.setor.h && filtroAtual.setor.v[0] != v[0]) {
			setTimeout(() => setorToggleEl.click(), 1);
		}
		algoModificado();
	}
	onMount(() => {
		carregarGastos();
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
						bind:value={filtroAtual.dataInicial.v}
						bind:this={dataInicialEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={filtroAtual.dataInicial.h}
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
						bind:value={filtroAtual.dataFinal.v}
						bind:this={dataFinalEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={filtroAtual.dataFinal.h}
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
						bind:valor={filtroAtual.fornecedor.v}
						bind:this={fornecedorEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={filtroAtual.fornecedor.h}
				/>
			</div>
		</div>
		<div>
			<h3>Empresa</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputEmpresa
						onEdit={empresaModificada}
						bind:valor={filtroAtual.empresa.v}
						bind:this={empresaEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={(e) => {
						if (e.currentTarget.checked) {
							empresaModificada(filtroAtual.empresa.v);
						}

						selecionarFiltro(e);
					}}
					bind:checked={filtroAtual.empresa.h}
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
						bind:valor={filtroAtual.setor.v}
						bind:this={setorEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={(e) => {
						if (e.currentTarget.checked) {
							setorModificado(filtroAtual.setor.v);
						}
						selecionarFiltro(e);
					}}
					bind:checked={filtroAtual.setor.h}
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
						bind:valor={filtroAtual.pagamento.v}
						bind:this={pagamentoEl}
					/>
				</div>
				<input
					type="checkbox"
					on:change={selecionarFiltro}
					bind:checked={filtroAtual.pagamento.h}
				/>
			</div>
		</div>
		<div>
			<h3>Caixa de entrada</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputCaixa
						onEdit={algoModificado}
						bind:valor={filtroAtual.caixa.v}
						bind:this={caixaEl}
					/>
				</div>
				<input type="checkbox" on:change={selecionarFiltro} bind:checked={filtroAtual.caixa.h} />
			</div>
		</div>
		<button on:click={reset}>Remover Filtros</button>
		<button on:click={carregarGastos}>Buscar</button>
	</div>
	<div class="table-holder">
		<table>
			<tr>
				<th>Data</th>
				<th>Fornecedor</th>
				<th>Empresa</th>
				<th>Setor</th>
				<th>Valor</th>
				<th>Pagamento</th>
				<th>NF</th>
				<th>Caixa</th>
				<th>Observações</th>
			</tr>
			{#each gastosFiltrados as gasto}
				<tr>
					<td>{gasto.data}</td>
					<td>{gasto.fornecedor}</td>
					<td>{gasto.empresa}</td>
					<td>{gasto.setor}</td>
					<td>{gasto.valor}</td>
					<td>{gasto.pagamento}</td>
					<td>{gasto.nf}</td>
					<td>{gasto.caixa}</td>
					<td>{gasto.obs}</td>
				</tr>
			{/each}
		</table>
	</div>
</main>

<style>
	main {
		display: flex;
		width: 100%;
		height: 100%;
		border: 1px dashed black;
	}
	.filtro {
		width: 150px;
		background-color: white;
		padding: 5px;
	}
	.filtro h3 {
		margin: 0;
		flex-grow: 1;
		width: 100%;
	}
	.filtro > div {
		display: flex;
		flex-direction: column;
		display: inline;
	}
	.controls-holder {
		display: flex;
		justify-content: space-between;
	}
	.input-holder {
		flex-grow: 1;
		flex-shrink: 1;
		max-width: calc(100% - 26px);
		display: flex;
		position: relative;
		--blur-amount: 1px;
		--opacity: 0.5;
		--pointer-events: all;
	}
	.input-holder::after {
		content: '';
		position: absolute;
		left: 0;
		right: 0;
		width: 100%;
		height: 100%;
		backdrop-filter: blur(var(--blur-amount));
		background-color: rgba(255, 255, 255, var(--opacity));
		pointer-events: var(--pointer-events);
	}

	.table-holder {
		flex-grow: 1;
		overflow: auto;
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
		background-color: white;
		white-space: nowrap;
		overflow: hidden;
		padding: 0.25rem;
	}
	th {
		position: sticky;
		top: 0; /* Don't forget this, required for the stickiness */
		border-top: 2px solid;
		border-bottom: 2px solid;
		border-right: 2px solid;
	}
	td{
		border-bottom: 1px solid;
  		border-right: 1px solid;
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
