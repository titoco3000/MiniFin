<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputNF from '$lib/components/InputNF.svelte';
	import InputValor from '$lib/components/InputValor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import InputData from './InputData.svelte';
	import ResultBox from '$lib/components/ResultBox.svelte';
	import { enviarNovoGasto, type Gasto } from '$lib/armazenamento';
	let resultBoxEl: ResultBox;

	let inputFornecedor: InputFornecedor;
	let inputNF: InputNF;
	let inputData: InputData;
	let inputValor: InputValor;
	let inputSetor: InputSetor;
	let inputCaixa: InputCaixa;
	let inputPagamento: InputTipoPagamento;
	let inputObs: HTMLTextAreaElement;

	function formSubmit(evento: { preventDefault: () => void }) {
		evento.preventDefault();
		let gasto: Gasto = {
			fornecedor: inputFornecedor.obterValor(),
			nf: inputNF.obterValor(),
			data: inputData.value,
			modificado: '',
			valor: inputValor.obterValor(),
			empresa: inputSetor.obterValor()[0],
			setor: inputSetor.obterValor()[1],
			caixa: inputCaixa.obterValor()[0],
			pagamento: inputPagamento.obterValor()[0],
			obs: inputObs.value
		};
		console.log('enviando gasto');

		enviarNovoGasto(gasto).then((resposta) => {
			if (resposta.Err) {
				let r = resposta.Err.join('\n');
				console.log(r);
				resultBoxEl.mensagem(r, 'ruim');
			}
			if(resposta.Ok){
				resposta.Ok.unshift("Sucesso!");
				let r = resposta.Ok.join('\n');
				console.log(r);
				resultBoxEl.mensagem(r, 'bom');
				
				inputValor.reset();
				inputData.reset();
				inputNF.reset();
				inputObs.value = '';
				
			}
			document.getElementById("result-box")?.scrollIntoView({ behavior: "smooth" });
			console.log('scrolling into');
			
		});
	}
</script>

<main>
	<form on:submit={formSubmit}>
		<section>
			<!-- svelte-ignore a11y-div-has-associated-control -->
			<div class="medium">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Fornecedor</label>
				<div>
					<InputFornecedor bind:this={inputFornecedor} />
				</div>
			</div>
			<div class="medium">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Nota Fiscal</label>
				<div>
					<InputNF bind:this={inputNF} />
				</div>
			</div>
			<div class="small">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Data</label> 
				<div>
					<InputData bind:this = {inputData}/>
				</div>
			</div>
			<div class="small">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Valor</label>
				<div>
					<InputValor bind:this={inputValor} />
				</div>
			</div>
			
			<div class="medium">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Setor</label>
				<div>
					<InputSetor bind:this={inputSetor} />
				</div>
			</div>
			
			<div class="medium">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Caixa de Entrada</label>
				
				<div>
					<InputCaixa bind:this={inputCaixa} />
				</div>
			</div>
			<div class="medium">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Tipo de Pagamento</label>
				<div>
					<InputTipoPagamento bind:this={inputPagamento} />
				</div>
			</div>
			<div class="large">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Observações</label>
				<div class="text-area-holder">
					<textarea bind:this={inputObs} />
				</div>
			</div>
		</section>
		<input type="submit" value="Confirmar" />
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
		width: 90%;
		max-width: 900px;
		margin: auto;
		margin-top: 40px;
	}
	#result-box {
		padding: 0;
	}
	form {
		padding-left: 8%;
		padding-right: 8%;
		background-color: var(--cor-tema-fundo-2);
		position: relative;
		border-radius: 5px;
		
		/* padding: 10%; */
		text-align: center;
	}
	form, input[type='submit']{
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5);
	}
	section {
		display: flex;
		flex-flow: row wrap;
	}
	.small {
		width: 25%;
		min-width: 150px;
	}
	.medium {
		width: 50%;
	}
	.large {
		width: 100%;
	}
	input[type='submit'] {
		font-size: 22px;
		background-color: var(--cor-tema-forte);
		border: none;
		border-radius: var(--tema-border-radius);
		padding: 10px;
		margin-top: 50px;
		margin-bottom: 30px;
	}
	div {
		box-sizing: border-box;
		margin-top: 0px;
		font-size: 16px;
		text-align: left;
		padding: 10px;
	}
	div > div {
		padding: 0px;
		box-sizing: border-box;
		width: 100%;
		height: 40px;
		margin: 0;
	}
	.text-area-holder {
		height: 120px;
	}
	textarea {
		padding: 10px;
		margin: 0;
		width: 100%;
		height: 100%;
		resize: none;
		box-sizing: border-box;
		border: 2px solid var(--cor-tema-detalhes);
		border-radius: var(--tema-border-radius);
		outline: none;
		/* border-width: 0px; */
	}
</style>
