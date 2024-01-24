<script lang="ts">
    import { onMount } from "svelte";
    
    export let value = new Date().toISOString().split('T')[0];
    export let onChange = ()=>{};
    
    let valueVisivel = '';

    export function reset(){
        value = new Date().toISOString().split('T')[0];
        atualizarVisivel()
    }

    function formatar(data:string){
        let partes = data.split('-');
        return partes[2]+'/'+partes[1]+'/'+partes[0];
    }
    function atualizarVisivel(){
        valueVisivel = formatar(value);
    }

    onMount(()=>{atualizarVisivel()});
</script>
<main>
    <input type="date" name="data" bind:value={value} on:change={()=>{
        atualizarVisivel();
        onChange();
    }}>
    <label for="data">{valueVisivel}</label>
</main>
<style>
    main{
        width: 100%;
        height: 40px;
        position: relative;
    }
    input{
        height: 100%;
        width: 100%;
        user-select: none;
    }
    label{
        font-size: 16px;
        display: flex;
        justify-content: space-around;
        align-self: center;
        height: 100%;
        width: 100%;
        position: absolute;
        top: 0;
        left: 0;
        background-color: var(--cor-tema-fundo-2);
        border: 2px solid var(--cor-tema-detalhes);
        border-radius: var(--tema-border-radius);
        pointer-events: none;
    }
</style>