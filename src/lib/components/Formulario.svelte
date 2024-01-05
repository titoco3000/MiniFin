<script lang="ts">
    import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputNF from '$lib/components/InputNF.svelte';
	import InputValor from '$lib/components/InputValor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputTipoPagamento from '$lib/components/InputTipoPagamento.svelte';
	import ResultBox from '$lib/components/ResultBox.svelte';

    let resultBoxEl:ResultBox;

    function formSubmit(evento: { preventDefault: () => void; }){
        evento.preventDefault();

    }
</script>
<main>

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
</main>
<style>
    main{
        width: 100%;
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