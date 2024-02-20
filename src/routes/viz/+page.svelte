<script lang="ts">
	import TopHeader from '$lib/components/TopHeader.svelte';
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputEmpresa from '$lib/components/InputEmpresa.svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import InputData from '$lib/components/InputData.svelte';
	import ScrollBox from '$lib/components/ScrollBox.svelte';
	import {
		listarGastos,
		FiltroGastos as Filtro,
		contarGastos,
		somarGastos,
		exportarParaXlsx
	} from '$lib/armazenamento';
	import { onMount } from 'svelte';
	import { formatarValor, formatarNF, formatarData } from '$lib/utils';
	import LazyTable from '$lib/components/LazyTable.svelte';

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

	//data de 1 mes atras
	let dataInicial = new Date();
	dataInicial.setMonth(dataInicial.getMonth() - 1);
	let dataFinal = new Date();

	let filtroAplicado = new Filtro(
		dataInicial.toISOString().split('T')[0],
		dataFinal.toISOString().split('T')[0]
	);
	let filtroAtual = new Filtro(
		dataInicial.toISOString().split('T')[0],
		dataFinal.toISOString().split('T')[0]
	);
	let valoresReais = new Filtro(
		dataInicial.toISOString().split('T')[0],
		dataFinal.toISOString().split('T')[0]
	);

	let valorToggle = {
		data_inicial: true,
		data_final: true,
		fornecedor: false,
		empresa: false,
		setor: false,
		tipo_pagamento: false,
		caixa: false,
		conteudo: false
	};

	let filtroEl: HTMLElement;
	let setorToggleEl: HTMLInputElement;
	let empresaToggleEl: HTMLInputElement;
	let togglePesquisaConteudo: HTMLInputElement;
	let lazyTableEl: LazyTable;
	let toggleDataEl: (HTMLInputElement | null)[] = [null, null];

	let dataInicialEl: InputData;
	let dataFinalEl: InputData;
	let fornecedorEl: { reset: Function };
	let empresaEl: { reset: Function };
	let setorEl: { reset: Function };
	let pagamentoEl: { reset: Function };
	let caixaEl: { reset: Function };
	let conteudoEl: HTMLInputElement;

	let somatorioValor = '0';

	function selecionarFiltro(e: any) {
		algoModificado();
		let el = e.srcElement.parentNode.parentNode.querySelector('.input-holder');
		setFiltroState(el,e.currentTarget.checked);
	}

	function setFiltroState(el:HTMLElement,state:boolean){
		if (state) {
			el.style.setProperty('--opacity', '0');
			el.style.setProperty('--blur-amount', '0px');
			el.style.setProperty('--pointer-events', 'none');
		} else {
			el.style.setProperty('--opacity', '0.5');
			el.style.setProperty('--blur-amount', '10px');
			el.style.setProperty('--pointer-events', 'all');
		}
	}

	function algoModificado() {
		//espera 1ms antes de agir para dar tempo de mudanças fazerem efeitos
		setTimeout(() => {
			//copia valores relevantes de valoresReais para filtroAtual
			for (const [key, value] of Object.entries(valorToggle)) {
				if (value)
					// @ts-ignore
					filtroAtual[key] = valoresReais[key];
				// @ts-ignore
				else filtroAtual[key] = [''];
			}
			if (!filtroAplicado.equals(filtroAtual)) {
				// console.log('diferentes');
			}
		}, 0);
	}

	function reset() {
		console.log('resetting');

		dataInicialEl.reset();
		dataFinalEl.reset();
		fornecedorEl.reset();
		empresaEl.reset();
		setorEl.reset();
		pagamentoEl.reset();
		caixaEl.reset();
		conteudoEl.value = '';
		setTimeout(() => {
			filtroEl.querySelectorAll<HTMLInputElement>("input[type='checkbox']").forEach((e) => {
					if (e.checked) e.click();
			});
			carregarGastosComNovoFiltro();
		}, 1);
	}

	function carregarGastosComNovoFiltro() {
		//copia filtro
		filtroAplicado = Object.assign(new Filtro(), JSON.parse(JSON.stringify(filtroAtual)));
		somarGastos(filtroAplicado).then((v) => {
			somatorioValor = formatarValor(v);
		});
		lazyTableEl.reset();
	}
	function setorModificado(v: string[]) {
		if (
			filtroAtual.empresa.length > 0 &&
			filtroAtual.empresa[0] != '' &&
			filtroAtual.empresa[0] != v[0]
		) {
			setTimeout(() => empresaToggleEl.click(), 1);
		}
		algoModificado();
	}
	function empresaModificada(v: string[]) {
		if (
			filtroAtual.setor.length > 0 &&
			filtroAtual.setor[0] != '' &&
			filtroAtual.setor[0] != v[0]
		) {
			setTimeout(() => setorToggleEl.click(), 1);
		}
		algoModificado();
	}

	function contarRows() {
		console.log('contando com filtro ',filtroAplicado);
		
		return contarGastos(filtroAplicado);
	}

	async function carregarValores(
		offset: number,
		limit: number,
		sorterIndex: number,
		sorterReverse: boolean
	): Promise<string[][]> {
		let resposta: string[][] = [];
		(
			await listarGastos(filtroAplicado, { i: sorterIndex, d: sorterReverse }, limit, offset)
		).forEach((gasto) => {
			resposta.push([
				formatarData(gasto.data),
				gasto.fornecedor,
				gasto.empresa,
				gasto.setor,
				formatarValor(gasto.valor),
				gasto.pagamento,
				formatarNF(gasto.nf),
				gasto.caixa,
				gasto.obs
			]);
		});
		return resposta;
	}

	function KeyPress(e: KeyboardEvent) {
		if (e) {
			if (e.key == 'f' && e.ctrlKey) {
				e.preventDefault();
				conteudoEl.focus();
				conteudoEl.scrollIntoView();
				if (!valorToggle.conteudo) {
					togglePesquisaConteudo.click();
				}
			}
		}
	}

	document.onkeydown = KeyPress;

	onMount(() => {
		// reset(true);
	});
</script>

<main>
	<TopHeader inicial={1} />
	<div id="content">
		<div class="filtro" bind:this={filtroEl}>
			<h2>Filtros</h2>
			<div id="box-holder">
				<ScrollBox>
					<section>
						<div class="controls-header">
							<h3>Data inicial</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:checked={valorToggle.data_inicial}
								bind:this={toggleDataEl[0]}
							/>
						</div>
						<div class="input-holder">
							<InputData
								onChange={algoModificado}
								bind:valor={valoresReais.data_inicial[0]}
								bind:this={dataInicialEl}
								placeholder={dataInicial.toISOString().split('T')[0]}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Data final</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:checked={valorToggle.data_final}
								bind:this={toggleDataEl[1]}
							/>
						</div>
						<div class="input-holder">
							<InputData
								onChange={algoModificado}
								bind:valor={valoresReais.data_final[0]}
								bind:this={dataFinalEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Fornecedor</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:checked={valorToggle.fornecedor}
							/>
						</div>
						<div class="input-holder">
							<InputFornecedor
								onEdit={algoModificado}
								permitirNovo={false}
								bind:valor={valoresReais.fornecedor[0]}
								bind:this={fornecedorEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Empresa</h3>
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
						<div class="input-holder">
							<InputEmpresa
								onEdit={empresaModificada}
								bind:valor={valoresReais.empresa}
								bind:this={empresaEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Setor</h3>
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
						<div class="input-holder">
							<InputSetor
								onEdit={setorModificado}
								bind:valor={valoresReais.setor}
								bind:this={setorEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Tipo de pagamento</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:checked={valorToggle.tipo_pagamento}
							/>
						</div>
						<div class="input-holder">
							<InputTipoPagamento
								onEdit={algoModificado}
								bind:valor={valoresReais.tipo_pagamento}
								bind:this={pagamentoEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Caixa de entrada</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:checked={valorToggle.caixa}
							/>
						</div>
						<div class="input-holder">
							<InputCaixa
								onEdit={algoModificado}
								bind:valor={valoresReais.caixa}
								bind:this={caixaEl}
							/>
						</div>
					</section>
					<section>
						<div class="controls-header">
							<h3>Geral</h3>
							<input
								type="checkbox"
								on:change={selecionarFiltro}
								bind:this={togglePesquisaConteudo}
								bind:checked={valorToggle.conteudo}
							/>
						</div>
						<div class="input-holder">
							<input type="text" bind:this={conteudoEl} bind:value={valoresReais.conteudo[0]} />
						</div>
					</section>
					<section id="buttons-holder">
						<button type="button" on:click={() => reset()}>Remover Filtros</button>
						<button type="button" on:click={carregarGastosComNovoFiltro}>Buscar</button>
					</section>
				</ScrollBox>
			</div>
		</div>
		<div class="table-holder">
			<LazyTable
				bind:this={lazyTableEl}
				{titulos}
				{carregarValores}
				calcularMaxRows={contarRows}
				exportar={(sorterIndex, reverse) => {
					exportarParaXlsx(filtroAplicado, { i: sorterIndex, d: reverse });
				}}
				bind:valorInferior={somatorioValor}
			/>
		</div>
	</div>
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
	}
	#content {
		height: 100%;
		padding: 10px 0 10px 10px;
		overflow: hidden;
		display: flex;
		width: 100%;
	}
	.filtro {
		flex: 1 0 160px;
		border: 2px solid black;
		background-color: var(--cor-tema-fraca);
		overflow: visible;
		display: flex;
		flex-direction: column;
	}
	.filtro h2 {
		background-color: var(--cor-tema-forte);
		margin: 0;
		padding: 5px;
		font-weight: 100;
		font-size: 22px;
	}
	#box-holder {
		width: 100%;
		overflow: visible;
		flex-grow: 1;
	}
	.filtro h3 {
		margin: 0;
		flex-grow: 1;
		width: 100%;
		font-weight: 600;
		font-size: 13.5px;
	}
	section {
		width: 100%;
		max-width: 156px;
		padding: 0 5px;
		pointer-events: all;
		margin: calc(calc(100vh - 200px) / 50) 0;
	}
	#buttons-holder {
		flex-direction: row;
		padding: 0 7px;
	}
	#buttons-holder > button:first-child {
		margin-bottom: 6px;
	}

	.controls-header {
		display: flex;
		align-items: center;
		margin: 1px 0;
	}
	input[type='checkbox'] {
		min-width: 18px;
		min-height: 18px;
		position: relative;
		margin: 0;
	}
	input[type='checkbox']::before {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		content: '';
		background-color: white;
		border: 2px solid black;
	}
	input[type='checkbox']::after {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		content: '';
		background-color: var(--cor-tema-fundo-1);
		border-radius: 10%;
		margin: 100%;
		transition: margin 0.3s;
	}
	input[type='checkbox']:checked::after {
		margin: 3px;
	}

	.input-holder {
		flex-grow: 1;
		flex-shrink: 1;
		position: relative;
		padding: 0px;
		width: 100%;
		height: var(--tema-altura-input);
		margin: 0;
		min-width: 0;
		--blur-amount: 10px;
		--opacity: 0.5;
		--pointer-events: all;
	}
	input[type='text'] {
		width: 100%;
		border: 2px solid black;
		border-radius: var(--tema-border-radius);
		height: var(--tema-altura-input);
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
		background-color: rgba(255, 255, 255, 0.9);
		transition:
			background-color 0.3s,
			backdrop-filter 0.3s,
			-webkit-backdrop-filter 0.3s;
	}
	section:first-child .input-holder, section:nth-child(2) .input-holder{
		--blur-amount: 0px;
		--opacity: 0;
		--pointer-events: none;
	}

	/* if backdrop support: very transparent and blurred */
	@supports ((-webkit-backdrop-filter: none) or (backdrop-filter: none)) {
		.input-holder::after {
			background-color: rgba(255, 255, 255, var(--opacity));
			backdrop-filter: blur(var(--blur-amount));
			-webkit-backdrop-filter: blur(var(--blur-amount));
		}
	}

	.filtro button {
		background-color: var(--cor-tema-forte);
		border-radius: var(--tema-border-radius);
		border: none;
	}

	.table-holder {
		flex-grow: 1;
		overflow: auto;
	}
</style>
