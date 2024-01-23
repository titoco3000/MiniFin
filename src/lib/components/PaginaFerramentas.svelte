<script lang="ts">
	import ImportadorCSV from "./ImportadorCSV.svelte";

    let contentHolder:HTMLElement;
    let buttonsHolder:HTMLElement;

    function revelarFerramenta(index:number){
        for (let i = 0; i < contentHolder.children.length; i++) {
            if(i == index){
                (buttonsHolder.children[i] as HTMLElement).style.backgroundColor = "var(--cor-tema-forte)";
                (contentHolder.children[i] as HTMLElement).style.display = "block";
            }
            else{
                (buttonsHolder.children[i] as HTMLElement).style.backgroundColor = "var(--cor-tema-fraca)";
                (contentHolder.children[i] as HTMLElement).style.display = "none";
            }
        }
    }
</script>

<main>
    <nav bind:this={buttonsHolder}>
        <button on:click={()=>revelarFerramenta(0)}>Importar CSV</button>
        <button on:click={()=>revelarFerramenta(1)}>Editar gasto</button>
        <button on:click={()=>revelarFerramenta(2)}>Editar fornecedor</button>
    </nav>
    <div class="content" bind:this={contentHolder}>
        <div>
            <ImportadorCSV />
        </div>
        <div></div>
        <div></div>
    </div>
</main>

<style>
    main{
        display: flex;
        width: 100%;
        height: 100%;
    }
    nav{
        background-color: var(--cor-tema-fundo-1);
        display: flex;
        flex-direction: column;
    }
    nav button{
        background-color: var(--cor-tema-fraca);
        border: none;
        border-radius: var(--tema-border-radius);
        margin: 10px;
    }
    nav > button:first-child{
        background-color: var(--cor-tema-forte);
    }
    .content{
        border: 2px dashed black;
        height: 100%;
        width: 100%;
    }
    .content > div{
        display: none;
    }
    .content > div:first-child{
        display: block;
    }
</style>
