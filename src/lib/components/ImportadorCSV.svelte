<script lang="ts">
	import { importarCSVs} from '$lib/armazenamento';
    
	let inputArquivoFornecedor: HTMLInputElement;
	let inputArquivoGasto: HTMLInputElement;

	let arquivoFornecedor: string = '';
	let arquivoGasto: string = '';

	function importar() {
		if (inputArquivoFornecedor.files) {
			var reader = new FileReader();
			reader.readAsText(inputArquivoFornecedor.files[0], 'UTF-8');
			reader.onload = (evt) => {
				let fornecedores = evt.target?.result;
				if (inputArquivoGasto.files) {
					reader.readAsText(inputArquivoGasto.files[0], 'UTF-8');
					reader.onload = (evt) => {
						let gastos = evt.target?.result;
                        if(typeof fornecedores === 'string' && typeof gastos === 'string'){
                            importarCSVs(fornecedores, gastos).then((resultado)=>{
                                console.log(resultado);
                                
                            });
                        }
					};
				}
			};
		}
	}

	function onChange(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let filename = e.currentTarget.value.split(/(\\|\/)/g).pop();
		if (filename) {
			if (e.currentTarget == inputArquivoFornecedor) arquivoFornecedor = filename;
			else arquivoGasto = filename;
		} else {
			if (e.currentTarget == inputArquivoFornecedor) arquivoFornecedor = '';
			else arquivoGasto = '';
		}
	}
</script>

<form on:submit={importar}>
	<div>
		<label>
			<h2>Arquivo fornecedores</h2>
			<input type="file" on:change={onChange} bind:this={inputArquivoFornecedor} />
			<p>{arquivoFornecedor}</p>
		</label>
		<label>
			<h2>Arquivo gastos</h2>
			<input type="file" on:change={onChange} bind:this={inputArquivoGasto} />
			<p>{arquivoGasto}</p>
		</label>
	</div>
	<input type="submit" />
</form>

<style>
	div {
		display: flex;
	}
	label {
		background-color: var(--cor-tema-fraca);
		width: 50%;
		margin: 10px;
		height: 200px;
		max-width: 200px;
		padding: 10px;
		cursor: pointer;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}
	p {
		text-align: right;
	}
	input[type='file'] {
		display: none;
	}
</style>
