<script lang="ts">
	import InputFornecedor from './InputFornecedor.svelte';
	import { renomearFornecedor, listarGastos, FiltroGastos, contarGastos } from '$lib/armazenamento';
    
    let confirmacaoEl:HTMLFormElement;
    
    let original:string;
    let novo:string;

    let qtdStr:string = '';

    const isWhitespaceString = (str:string) => !str.replace(/\s/g, '').length

    function onConfirmacao(e:SubmitEvent){
        e.preventDefault();
        if(e.submitter){
            let submitter = e.submitter as HTMLInputElement;
            if(submitter.value == "Cancelar"){
            }
            else{
                renomearFornecedor(original, novo);
            }
            confirmacaoEl.style.display='none';
        }
    }

</script>
<main>
    <form
        on:submit={(e) => {
            e.preventDefault();
            if(!isWhitespaceString(novo)){
                let filtro = new FiltroGastos();
                filtro.fornecedor = [original];
                contarGastos(filtro).then((qtd) => {
                    qtdStr = ''+qtd;
                });
                confirmacaoEl.style.display='block';
            }
        }}
    >
        <div>
            <p>Fornecedor</p>
            <span>
                <InputFornecedor bind:valor={original} permitirNovo={false}/>
            </span>
        </div>
        <div>
            <p>Novo nome</p>
            <span>
                <InputFornecedor bind:valor={novo} permitirNovo={true}/>
            </span>
        </div>
        <div id="buttons-holder">
            <input type="submit" value="Confirmar"/>
        </div>
    </form>
    <form bind:this={confirmacaoEl} id="confirmacao" on:submit={onConfirmacao}>
        <p>Deseja mesmo mudar o fornecedor <em>{original}</em> para <em>{novo}</em>?</p>
        <p>Todas as suas <strong>{qtdStr}</strong> compras serão movidas para este novo nome.</p>
        <div id="buttons-holder">
            <input type="submit" value="Cancelar"/>
            <input type="submit" value="Confirmar"/>
        </div>
    </form>
</main>


<style>
    main{
        height: 100%;
        width: 100%;
        border: 2px dashed black;
        display: flex;
        padding: 20px;
        position: relative;
    }
	form {
		height: 100%;
		width: 100%;
		background-color: var(--cor-tema-fraca);
		padding: 20px;
		border: 2px solid black;
		border-radius: var(--tema-border-radius);
        max-width: 400px;
	}
	input {
        border: 2px solid black;
		border-radius: var(--tema-border-radius);
	}
    #buttons-holder{
        margin-top: 10px;
    }
    input:first-child{
        background-color: rgb(255, 157, 157);
    }
    input:last-child{
        background-color: rgb(115, 253, 142);
    }
    #confirmacao{
        position: absolute;
        top: 40px;
        bottom: 40px;
        left: 40px;
        right: 40px;
        width: auto;
        height: auto;
        background-color: white;
        display: none;
    }
</style>
