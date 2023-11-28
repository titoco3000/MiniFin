<script lang="ts">
    import InputFornecedor from "$lib/components/InputFornecedor.svelte";
    import InputNF from "$lib/components/InputNF.svelte";
    import InputValor from "$lib/components/InputValor.svelte";
    import InputSetor from "$lib/components/InputSetor.svelte";
    import InputCaixa from "$lib/components/InputCaixa.svelte";
    import TabelaGastos from "$lib/components/TabelaGastos.svelte";
    let currentEl:any;
</script>
<h1>Bem vindo aos testes</h1>
<button on:click={()=>console.log(currentEl.obterValor())
}>testar</button>
<div class="container-input-fornecedor">
    <!-- <InputFornecedor/> -->
    <!-- <br>
    <InputNF/>
    <br>
    <InputValor/>
    <br>
    <InputSetor/>
    <br>
    <InputCaixa bind:this={currentEl}/> -->
    <TabelaGastos/>
</div>



<style>
.container-input-fornecedor{
    width: 800px;
    height: 500px;
    resize: both;
    overflow: hidden;
    background-color:rgb(135, 135, 185);
    padding: 10px;
}
</style>
