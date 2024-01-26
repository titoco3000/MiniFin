<script lang="ts">
    import PaginaFerramentas from "$lib/components/PaginaFerramentas.svelte";
    import TopHeader from "$lib/components/TopHeader.svelte";
</script>
<main>
    <TopHeader inicial={2}/>
    <PaginaFerramentas />
</main>
<style>
    main{
        width: 100vw;
        height: 100vh;
    }
</style>