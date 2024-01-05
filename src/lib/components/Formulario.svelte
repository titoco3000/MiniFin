<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputNF from '$lib/components/InputNF.svelte';
	import InputValor from '$lib/components/InputValor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import ResultBox from '$lib/components/ResultBox.svelte';

	let resultBoxEl: ResultBox;

	let inputFornecedor: InputFornecedor;
	let inputNF: InputNF;
	let inputData: HTMLInputElement;
	let inputValor: InputValor;
	let inputSetor: InputSetor;
	let inputCaixa: InputCaixa;
	let inputPagamento: InputTipoPagamento;
	let inputObs: HTMLTextAreaElement;

	function formSubmit(evento: { preventDefault: () => void }) {
		evento.preventDefault();
		let gasto = {
			fornecedor: inputFornecedor.obterValor(),
			NF: inputNF.obterValor(),
			data: inputData.value,
			valor: inputValor.obterValor(),
			setor: inputSetor.obterValor(),
			caixa: inputCaixa.obterValor()[0],
			tipo_de_pagamento: inputPagamento.obterValor()[0],
			obs: inputObs.value
		};
		console.log(JSON.stringify(gasto));
	}
</script>

<main>
	<form on:submit={formSubmit}>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label>
			Fornecedor
			<InputFornecedor bind:this={inputFornecedor} />
		</label>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="input-nf-holder">
			Nota Fiscal
			<InputNF bind:this={inputNF} />
		</label>
		<label>
			Data
			<input type="date" name="" id="" bind:this={inputData} />
		</label>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label>
			Valor
			<InputValor bind:this={inputValor} />
		</label>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label>
			Setor
			<InputSetor bind:this={inputSetor} />
		</label>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label>
			Caixa de entrada
			<InputCaixa bind:this={inputCaixa} />
		</label>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label>
			Tipo de Pagamento
			<InputTipoPagamento bind:this={inputPagamento} />
		</label>
		<label>
			Observações
			<textarea bind:this={inputObs} />
		</label>
		<label>
			<input type="submit" value="Confirmar" />
		</label>
	</form>
	<div id="result-box">
		<ResultBox bind:this={resultBoxEl} />
	</div>
</main>

<style>
	main {
		width: 100%;
	}
	form,
	#result-box {
		box-sizing: border-box;
		width: 100%;
		max-width: 600px;
		padding: 10px;
		margin: auto;
	}
	#result-box {
		margin-top: 20px;
		padding: 0;
	}
	form {
		background-color: rgb(159, 159, 159);
	}
	form > label {
		margin: 10px;
	}
	.input-nf-holder {
		width: 10em;
	}
	textarea {
		width: 100%;
		resize: none;
	}
</style>
