<script lang="ts">
	import TopHeader from '$lib/components/TopHeader.svelte';
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputEmpresa from '$lib/components/InputEmpresa.svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import InputData from '$lib/components/InputData.svelte';
	import {
		listarGastos,
		type Gasto,
		FiltroGastos as Filtro,
		contarGastos,
		somarGastos
	} from '$lib/armazenamento';
	import { onMount } from 'svelte';
	import { obterDigitosNF } from '$lib/utils';
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
		caixa: false,
		conteudo: false
	};

	let filtroEl: HTMLElement;
	let setorToggleEl: HTMLInputElement;
	let empresaToggleEl: HTMLInputElement;
    let togglePesquisaConteudo: HTMLInputElement;
	let lazyTableEl: LazyTable;
	
	let dataInicialEl: InputData;
	let dataFinalEl: InputData;
	let fornecedorEl: { reset: Function };
	let empresaEl: { reset: Function };
	let setorEl: { reset: Function };
	let pagamentoEl: { reset: Function };
	let caixaEl: { reset: Function };
	let conteudoEl: HTMLInputElement;

	let somatorioValor = '0';
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
			console.log('valoresReais', valoresReais);

			filtroEl.querySelectorAll<HTMLInputElement>("input[type='checkbox']").forEach((e) => {
				if (e.checked) {
					e.click();
				}
			});
			carregarGastosComNovoFiltro();
		}, 1);
	}

	function carregarGastosComNovoFiltro() {
		console.log('carregarGastosComNovoFiltro');

		//copia filtro
		filtroAplicado = Object.assign(new Filtro(), JSON.parse(JSON.stringify(filtroAtual)));
		somarGastos(filtroAplicado).then((v) => {
			somatorioValor = formatarValor(v);
		});
		lazyTableEl.reset();
	}
	function setorModificado(v: string[]) {
		if (filtroAtual.empresa.length > 0 && filtroAtual.empresa[0] != v[0]) {
			setTimeout(() => empresaToggleEl.click(), 1);
		}
		algoModificado();
	}
	function empresaModificada(v: string[]) {
		if (filtroAtual.setor.length > 0 && filtroAtual.setor[0] != v[0]) {
			setTimeout(() => setorToggleEl.click(), 1);
		}
		algoModificado();
	}

	function contarRows(){
		return contarGastos(filtroAplicado);
	}

	function formatarValor(v: number) {
		let s: string = `${v}`;
		while (s.length < 3) s = '0' + s;
		let inteira = s.slice(0, s.length - 2);
		let pontuada =
			inteira.slice(0, inteira.length % 3) +
			inteira.slice(inteira.length % 3).replace(/.{3}/g, '.$&');
		if (pontuada[0] == '.') {
			pontuada = pontuada.slice(1);
		}
		//.split('').reverse().join('').replace(/.{3}/g, '$&.').split('').reverse().join('')
		return pontuada + ',' + s.slice(s.length - 2);
		//	562 322 2,48
	}
	function formatarNF(nf: number) {
		let s: string = `${nf}`;
		while (s.length < digitosNF) s = '0' + s;
		return s;
	}
	function formatarData(dataStr:string){
		let s = dataStr.split('-');
		return s[2]+'/'+s[1]+'/'+s[0];
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
                if(!valorToggle.conteudo){
                    togglePesquisaConteudo.click();
                }
			}
		}
	}

	//data de 6 meses atras
	let dataInicial = new Date();
	dataInicial.setMonth(dataInicial.getMonth() - 6);

	digitosNF = obterDigitosNF();

    document.onkeydown = KeyPress;

	onMount(() => {
		reset();
	});
</script>

<main>
	<TopHeader inicial={1} />
	<section id="content">
        <div class="filtro" bind:this={filtroEl}>
            <h2>Filtros</h2>
            <div>
                <h3>Data inicial</h3>
                <div class="controls-holder">
                    <div class="input-holder">
                        <InputData
                            onChange={algoModificado}
                            bind:value={valoresReais.data_inicial[0]}
                            bind:this={dataInicialEl}
                            placeholder={dataInicial.toISOString().split('T')[0]}
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
                        <InputData
                            onChange={algoModificado}
                            bind:value={valoresReais.data_final[0]}
                            bind:this={dataFinalEl}
                        />
                    </div>
                    <input type="checkbox" on:change={selecionarFiltro} bind:checked={valorToggle.data_final} />
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
                    <input type="checkbox" on:change={selecionarFiltro} bind:checked={valorToggle.fornecedor} />
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
                        <InputCaixa onEdit={algoModificado} bind:valor={valoresReais.caixa} bind:this={caixaEl} />
                    </div>
                    <input type="checkbox" on:change={selecionarFiltro} bind:checked={valorToggle.caixa} />
                </div>
            </div>
            <div>
                <h3>Geral</h3>
                <div class="controls-holder">
                    <div class="input-holder">
                        <input type="text" bind:this={conteudoEl}
                        bind:value={valoresReais.conteudo[0]}
                        >
                    </div>
                    <input
                        type="checkbox"
                        on:change={selecionarFiltro}
                        bind:this={togglePesquisaConteudo}
                        bind:checked={valorToggle.conteudo}
                    />
                </div>
            </div>
            <div id="buttons-holder">
                <button type="button" on:click={reset}>Remover Filtros</button>
                <button type="button" on:click={carregarGastosComNovoFiltro}>Buscar</button>
            </div>
        </div>
        <div class="table-holder">
            <LazyTable
                bind:this={lazyTableEl}
                {titulos}
                {carregarValores}
                calcularMaxRows={contarRows}
                bind:valorInferior={somatorioValor}
            />
        </div>
    </section>
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
		flex: 1 0 180px;
		border: 2px solid black;
		background-color: var(--cor-tema-fraca);
		/* overflow-y: auto; */
        overflow: visible;
	}
	.filtro h2 {
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
		font-weight: 500;
	}
	.filtro > div {
		display: flex;
		flex-direction: column;
		/* border-bottom: 4px solid white; */
		width: 100%;
        max-width: 180px;
		padding: 5px;
	}
    #buttons-holder{
        flex-direction: row;
    }
    #buttons-holder > button:first-child{
        margin-right: 3px;
    }
	.controls-holder {
		display: flex;
		justify-content: space-between;
		width: 100%;
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
		--blur-amount: 1px;
		--opacity: 0.5;
		--pointer-events: all;
	}
	input[type='checkbox'] {
		margin-left: 10px;
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
