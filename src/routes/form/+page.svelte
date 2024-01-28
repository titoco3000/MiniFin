<script lang="ts">
    import Formulario from "$lib/components/Formulario.svelte";
    import TopHeader from "$lib/components/TopHeader.svelte";
</script>
<main>
    <TopHeader/>
    <Formulario />
</main>
<style>
    main{
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
    }
</style>