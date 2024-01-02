<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputNF from '$lib/components/InputNF.svelte';
	import InputValor from '$lib/components/InputValor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import TabelaGastos from '$lib/components/TabelaGastos.svelte';
	import { onMount } from 'svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import InputEmpresa from '$lib/components/InputEmpresa.svelte';
	import NewDropdown from '$lib/components/NewDropdown.svelte';

	let formSecEl: HTMLElement;
	let vizSecEl: HTMLElement;
	let ferramentasSecEl: HTMLElement;
	function revelarSection(sec: HTMLElement) {
		formSecEl.style.display = 'none';
		vizSecEl.style.display = 'none';
		ferramentasSecEl.style.display = 'none';

		sec.style.display = 'block';
	}
	onMount(() => {
		revelarSection(formSecEl);
	});
</script>

<main>
	<InputEmpresa/>
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
		<form action="">
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
	</section>
	<section bind:this={vizSecEl}>
		<TabelaGastos />
	</section>
	<section bind:this={ferramentasSecEl}>
	<NewDropdown/>
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
    form{
        box-sizing: border-box;
        width: 100%;
        max-width: 600px;
        padding: 10px;
        background-color: rgb(159, 159, 159);
        margin: auto;
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
