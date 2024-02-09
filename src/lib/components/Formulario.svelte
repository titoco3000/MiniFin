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
	import FormularioConfirmacao from './FormularioConfirmacao.svelte';

	let resultBoxEl: ResultBox;

	let inputFornecedor: InputFornecedor;
	let inputNF: InputNF;
	let inputData: InputData;
	let inputValor: InputValor;
	let inputSetor: InputSetor;
	let inputCaixa: InputCaixa;
	let inputPagamento: InputTipoPagamento;
	let inputObs: HTMLTextAreaElement;

	let formConfirmacao: FormularioConfirmacao;

	function formSubmit(evento: { preventDefault: () => void }) {
		evento.preventDefault();
		formConfirmacao.pedirConfirmacao(
			inputFornecedor.obterValor(),
			inputNF.obterValor(),
			inputData.obterValor(),
			inputValor.obterValor(),
			inputSetor.obterValor()[0],
			inputSetor.obterValor()[1],
			inputPagamento.obterValor()[0],
			inputCaixa.obterValor()[0],
			inputObs.value,
			(resultado: boolean) => {
				if (resultado) {
					let gasto: Gasto = {
						fornecedor: inputFornecedor.obterValor(),
						nf: inputNF.obterValor(),
						data: inputData.obterValor(),
						modificado: '',
						valor: inputValor.obterValor(),
						empresa: inputSetor.obterValor()[0],
						setor: inputSetor.obterValor()[1],
						caixa: inputCaixa.obterValor()[0],
						pagamento: inputPagamento.obterValor()[0],
						obs: inputObs.value
					};
					console.log('enviando gasto: ',gasto);

					enviarNovoGasto(gasto).then((resposta) => {
						if (resposta.Err) {
							let r = resposta.Err.join('\n');
							console.log(r);
							resultBoxEl.mensagem(r, 'ruim');
						}
						if (resposta.Ok) {
							resposta.Ok.unshift('Sucesso!');
							let r = resposta.Ok.join('\n');
							console.log(r);
							resultBoxEl.mensagem(r, 'bom');

							inputValor.reset();
							inputData.reset();
							inputNF.reset();
							inputObs.value = '';
						}
						document.getElementById('result-box')?.scrollIntoView(false);
						console.log('scrolling into');
					});
				}
			}
		);
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
					<InputData bind:this={inputData} />
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
	<FormularioConfirmacao bind:this={formConfirmacao} />
</main>

<style>
	main {
		width: 100%;
		height: 100%;
		position: relative;
		overflow: auto;
	}
	form,
	#result-box {
		box-sizing: border-box;
		width: 90%;
		max-width: 900px;
		margin: auto;
		margin-top: calc(var(--tema-altura-input) * 0.5);
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
	form,
	input[type='submit'] {
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5);
	}
	section {
		display: flex;
		flex-flow: row wrap;
	}
	.small {
		width: 25%;
		min-width: 90px;
	}
	@media screen and (max-width: 500px) {
		.small {
			width: 50%;
		}
	}
	.medium {
		width: 50%;
	}
	@media screen and (max-width: 320px) {
		.small, .medium {
			width: 100%;
		}
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
		margin: 10px;
		margin-bottom: 20px;
	}
	div {
		box-sizing: border-box;
		margin-top: 0px;
		font-size: 16px;
		text-align: left;
		padding: 10px 1% ;
	}
	@media screen and (max-width: 400px) {
		div {
			padding: 10px 1px;
		}
	}

	div > div {
		padding: 0px;
		box-sizing: border-box;
		width: 100%;
		height: var(--tema-altura-input);
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
