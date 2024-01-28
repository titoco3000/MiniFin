<script lang="ts">
	import { formatarValor, formatarNF, formatarData } from '$lib/utils';

	export function pedirConfirmacao(
		fornecedor: string,
		nf: number,
		data: string,
		valor: number,
		empresa: string,
		setor: string,
		pagamento: string,
		caixa: string,
		obs: string,
		callback: (confirmado: boolean) => void
	) {
		if (displays[0]) displays[0].innerText = fornecedor;
		if (displays[1]) displays[1].innerText = formatarNF(nf);
		if (displays[2]) displays[2].innerText = formatarData(data);
		if (displays[3]) displays[3].innerText = formatarValor(valor);
		if (displays[4]) displays[4].innerText = empresa;
		if (displays[5]) displays[5].innerText = setor;
		if (displays[6]) displays[6].innerText = pagamento;
		if (displays[7]) displays[7].innerText = caixa;
		if (displays[8]) displays[8].innerText = obs;
        funcaoCallback = callback;
        mainEl.style.display = "flex";
	}

    let mainEl:HTMLElement;
	let displays: (HTMLElement | null)[] = [null, null, null, null, null, null, null];
    let funcaoCallback:(confirmado: boolean)=>void;

	function onSubmit(e: SubmitEvent) {
        e.preventDefault();
		if(e.submitter){
            let submitter = e.submitter as HTMLInputElement;
            if(submitter.value == "Cancelar"){
                funcaoCallback(false);
            }
            else
                funcaoCallback(true);
        }
        mainEl.style.display = "none";        
	}
</script>

<main bind:this={mainEl}>
	<form on:submit={onSubmit}>
		<h1>Confirmação</h1>
		<div>
			<h2>Fornecedor:</h2>
			<p bind:this={displays[0]}></p>
		</div>
		<div>
			<h2>Nota Fiscal:</h2>
			<p bind:this={displays[1]}></p>
		</div>
		<div>
			<h2>Data:</h2>
			<p bind:this={displays[2]}></p>
		</div>
		<div>
			<h2>Valor:</h2>
			<p bind:this={displays[3]}></p>
		</div>
		<div>
			<h2>Empresa:</h2>
			<p bind:this={displays[4]}></p>
		</div>
		<div>
			<h2>Setor:</h2>
			<p bind:this={displays[5]}></p>
		</div>
		<div>
			<h2>Pagamento:</h2>
			<p bind:this={displays[6]}></p>
		</div>
		<div>
			<h2>Caixa:</h2>
			<p bind:this={displays[7]}></p>
		</div>
        <div>
			<h2>Observações:</h2>
			<p bind:this={displays[8]}></p>
		</div>
		<div id="botoes">
			<input type="submit" value="Cancelar" />
			<input type="submit" value="Confirmar" />
		</div>
	</form>
</main>

<style>
	main {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.349);
		align-items: center;
		justify-content: center;
		display: none;
	}
    /* if backdrop support: very transparent and blurred */
	@supports ((-webkit-backdrop-filter: none) or (backdrop-filter: none)) {
		main {
			backdrop-filter: blur(2px);
			-webkit-backdrop-filter: blur(2px);
		}
	}

	form {
		background-color: var(--cor-tema-fundo-2);
		padding: 0 40px 10px 40px;
		border-radius: var(--tema-border-radius);
        width: 90%;
        max-width: 600px;
        border: 2px solid black;
	}
	h1 {
		font-size: 40px;
	}
	div {
		display: flex;
	}
	div > * {
		margin: 3px;
	}
	p {
		flex-grow: 1;
		border-bottom: 1px solid black;
	}
	#botoes {
		justify-content: space-around;
	}
	#botoes > input {
		border-radius: var(--tema-border-radius);
	}
	#botoes > input:first-child {
		background-color: rgb(255, 157, 157);
	}
	#botoes > input:last-child {
		background-color: rgb(100, 255, 100);
	}
</style>
