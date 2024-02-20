<script lang="ts">
    import TopHeader from "$lib/components/TopHeader.svelte";
    import ExclusorGasto from "$lib/components/ExclusorGasto.svelte";
    import EditorFornecedor from "$lib/components/EditorFornecedor.svelte";

    let contentHolder:HTMLElement;
    let buttonsHolder:HTMLElement;

    function revelarFerramenta(index:number){
        for (let i = 0; i < buttonsHolder.children.length; i++) {
            if(i == index){
                (buttonsHolder.children[i] as HTMLElement).style.backgroundColor = "var(--cor-tema-forte)";
                (contentHolder.children[i] as HTMLElement).style.display = "flex";
            }
            else{
                (buttonsHolder.children[i] as HTMLElement).style.backgroundColor = "var(--cor-tema-fraca)";
                (contentHolder.children[i] as HTMLElement).style.display = "none";
            }
        }
    }

</script>
<main>
    <TopHeader inicial={2}/>
    <div id="content">
        <nav bind:this={buttonsHolder}>
            <button on:click={()=>revelarFerramenta(0)}>Excluir gasto</button>
            <button on:click={()=>revelarFerramenta(1)}>Editar fornecedor</button>
        </nav>
        <div id="container-ferramentas" bind:this={contentHolder}>
            <div>
                <div class="padded-holder">
                    <ExclusorGasto />
                </div>
            </div>
            <div>
                <EditorFornecedor />
            </div>
            <div></div>
        </div>
    </div>
</main>
<style>
    main{
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
    }
    #content{
        display: flex;
        padding: 10px;
        height: 100%;
    }
    #container-ferramentas{
        flex-grow: 1;
    }
    #container-ferramentas > div{
        display: none;
        width: 100%;
        height: 100%;
    }
    nav{
        display: flex;
        flex-direction: column;
        background-color: var(--cor-tema-fundo-1);
        height: 100%;
    }
    nav button{
        background-color: var(--cor-tema-fraca);
        border: none;
        border-radius: var(--tema-border-radius);
        margin: 10px;
    }
    .padded-holder{
        padding: 20px;
    }
</style>