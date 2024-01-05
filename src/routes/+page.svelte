<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputNF from '$lib/components/InputNF.svelte';
	import InputValor from '$lib/components/InputValor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import TabelaGastos from '$lib/components/TabelaGastos.svelte';
	import { onMount } from 'svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import ResultBox from '$lib/components/ResultBox.svelte';

	let formSecEl: HTMLElement;
	let vizSecEl: HTMLElement;
	let ferramentasSecEl: HTMLElement;
	let resultBoxEl: ResultBox;
	function revelarSection(sec: HTMLElement) {
		formSecEl.style.display = 'none';
		vizSecEl.style.display = 'none';
		ferramentasSecEl.style.display = 'none';

		sec.style.display = 'block';
	}
	function formSubmit(e: { preventDefault: () => void; }){
		e.preventDefault();
		console.log('submit');
		
	}
	onMount(() => {
		revelarSection(formSecEl);
	});
</script>
<main>
	<nav>
		<button
			on:click={() => {
				revelarSection(formSecEl);
			}}>Formulário</button
		>
		<button
			on:click={() => {
				revelarSection(vizSecEl);
			}}>Visualizar</button
		>
		<button
			on:click={() => {
				revelarSection(ferramentasSecEl);
			}}>Ferramentas</button
		>
	</nav>
	<section bind:this={formSecEl}>
		<form on:submit={formSubmit}>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				Fornecedor
				<InputFornecedor/>
			</label>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="input-nf-holder">
				Nota Fiscal
				<InputNF />
			</label>
			<label>
				Data
				<input type="date" name="" id="">
			</label>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				Valor
				<InputValor />
			</label>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				Setor
				<InputSetor />
			</label>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				Caixa de entrada
				<InputCaixa />
			</label>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				Tipo de Pagamento
				<InputTipoPagamento/>
			</label>
			<label>
				Observações
				<textarea/>
			</label>
			<label>
				<input type="submit" value="Confirmar">
			</label>
		</form>
		<div id="result-box">
			<ResultBox bind:this={resultBoxEl}/>
		</div>
	</section>
	<section bind:this={vizSecEl}>
		<TabelaGastos />
	</section>
	<section bind:this={ferramentasSecEl}>
	</section>
</main>

<style> 
	main {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
	}
	nav {
		background-color: rgb(96, 96, 96);
		margin: 0;
	}
	nav button {
		font-size: large;
	}
	section {
		display: none;
		flex: 1;
		overflow: auto;
		padding: 10px;
	}
	form, #result-box{
		box-sizing: border-box;
        width: 100%;
        max-width: 600px;
        padding: 10px;
		margin: auto;
	}
	#result-box{
		margin-top: 20px;
		padding: 0;
	}
    form{
        background-color: rgb(159, 159, 159);
    }
    form > label{
        margin: 10px;
    }
    .input-nf-holder{
        width: 10em;
    }
    textarea{
        width: 100%;
        resize: none;
    }
</style>
