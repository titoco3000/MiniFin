<script lang="ts">
    import TabelaGastos from "$lib/components/TabelaGastos.svelte";
    import TopHeader from "$lib/components/TopHeader.svelte";
</script>
<main>
    <TopHeader inicial={1}/>
    <div>
        <TabelaGastos />
    </div>
</main>
<style>
    main{
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
    }
    main>div{
        height: 100%;
        padding: 10px;
        overflow: hidden;
    }
</style>