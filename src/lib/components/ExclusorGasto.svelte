<script lang="ts">
	import InputFornecedor from './InputFornecedor.svelte';
	import InputNf from './InputNF.svelte';
	import FormularioConfirmacao from './FormularioConfirmacao.svelte';
	import { listarGastos, FiltroGastos, excluirGasto, type Gasto } from '$lib/armazenamento';
	import { onMount } from 'svelte';

	let formularioConfirmacao: FormularioConfirmacao;
	let fornecedor: string;
	let fornecedorEl: InputFornecedor;
	let nfEl: InputNf;
	let submitButtonEl: HTMLInputElement;

	let gastoAtual: Gasto | null = null;

	function algoModificado() {
		let filtro = new FiltroGastos();
        if(fornecedor!=''){
		    filtro.fornecedor = [fornecedor];
            listarGastos(filtro, { i: 0, d: false }, 999999, 0).then((gastos) => {
                console.log('pedi com ',filtro,', recebi ', gastos);
                let r = gastos.filter((g) => g.nf == nfEl.obterValor());
                if (r.length > 0) {
                    gastoAtual = r[0];
                    submitButtonEl.style.display = 'block';
                } else {
                    gastoAtual = null;
                    submitButtonEl.style.display = 'none';
                }
            });
        }
        else{
            setTimeout(algoModificado, 10);
        }
	}
	onMount(algoModificado);
</script>

<form
	on:submit={(e) => {
		e.preventDefault();
		if (gastoAtual != null)
			formularioConfirmacao.pedirConfirmacao(
				gastoAtual.fornecedor,
				gastoAtual.nf,
				gastoAtual.data,
				gastoAtual.valor,
				gastoAtual.empresa,
				gastoAtual.setor,
				gastoAtual.pagamento,
				gastoAtual.caixa,
				gastoAtual.obs,
				(confirmado) => {
					if (confirmado && gastoAtual != null) {
						excluirGasto(gastoAtual.fornecedor, gastoAtual.nf);
                        fornecedorEl.reset();
                        nfEl.reset();
					}
				}
			);
	    }
    }
>
	<div id="identificacao">
		<span>
			<InputFornecedor bind:this={fornecedorEl} permitirNovo={false} bind:valor={fornecedor} onEdit={algoModificado} />
		</span>
		<span>
			<InputNf bind:this={nfEl} onEdit={algoModificado} />
		</span>
	</div>
	<input type="submit" value="excluir" bind:this={submitButtonEl}/>
</form>
<FormularioConfirmacao bind:this={formularioConfirmacao} />

<style>
	form {
		height: 100%;
		width: 100%;
		background-color: var(--cor-tema-fraca);
		padding: 20px;
		border: 2px solid black;
		border-radius: var(--tema-border-radius);
	}
	#identificacao {
		display: flex;
		width: 50%;
		margin-bottom: 10px;
	}
	span {
		margin-right: 4px;
		width: 50%;
	}
	input {
		background-color: rgb(255, 157, 157);
		border: 2px solid black;
		border-radius: var(--tema-border-radius);
	}
</style>
