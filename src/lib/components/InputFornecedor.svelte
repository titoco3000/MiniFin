<script lang="ts">
	import { listarFornecedores } from '$lib/armazenamento';
	import { onMount } from 'svelte';

	export function obterValor(){
		return valor;
	}
	export function reset(){
		if(permitirNovo)
			valor = '';
		else{
			if(fornecedores.length>0)
				valor = fornecedores[0].nome;
			else
				valor = '';
		}
	}
	export let permitirNovo = true;

	export let valor: string = "";
	export let onEdit = (v:string)=>{};

	let fornecedores: any = [];
	let visualFornecedores: any = [];
	let mainEl: Node;
	let inputEl: HTMLInputElement;

	let maxFornecedoresExibidos = 5;

	function levDist(s: string, t: string): number {
		var d: number[][] = []; //2d matrix

		// Step 1
		var n = s.length;
		var m = t.length;

		if (n == 0) return m;
		if (m == 0) return n;

		//Create an array of arrays in javascript (a descending loop is quicker)
		for (var i = n; i >= 0; i--) d[i] = [];

		// Step 2
		for (var i = n; i >= 0; i--) d[i][0] = i;
		for (var j = m; j >= 0; j--) d[0][j] = j;

		// Step 3
		for (var i = 1; i <= n; i++) {
			var s_i = s.charAt(i - 1);

			// Step 4
			for (var j = 1; j <= m; j++) {
				//Check the jagged ld total so far
				if (i == j && d[i][j] > 4) return n;

				var t_j = t.charAt(j - 1);
				var cost = s_i == t_j ? 0 : 1; // Step 5

				//Calculate the minimum
				var mi = d[i - 1][j] + 1;
				var b = d[i][j - 1] + 1;
				var c = d[i - 1][j - 1] + cost;

				if (b < mi) mi = b;
				if (c < mi) mi = c;

				d[i][j] = mi; // Step 6

				//Damerau transposition
				if (i > 1 && j > 1 && s_i == t.charAt(j - 2) && s.charAt(i - 2) == t_j) {
					d[i][j] = Math.min(d[i][j], d[i - 2][j - 2] + cost);
				}
			}
		}

		// Step 7
		return d[n][m];
	}

	function assignDistancesAndSort(target: string) {
		for (let f of fornecedores) {
			f.levDist = levDist(f.nome, target);
		}
		fornecedores = fornecedores.sort((a: { levDist: number }, b: { levDist: number }) => {
			return a.levDist > b.levDist ? 1 : -1;
		});
		//aqui fornecedores está ordenado por levDist

		
		let A = [];
		let B = [];
		let C = [];
		
		for (let f of fornecedores) {
			if(f.nome.toLowerCase().startsWith(target.toLowerCase()))
				A.push(f);
			else if(f.nome.toLowerCase().includes(target.toLowerCase()))
				B.push(f);
			else
				C.push(f);
		}
		fornecedores = A.concat(B,C);

		return fornecedores;
	}

	function exibirTodosFornecedores() {
		visualFornecedores = fornecedores;
	}

	function ehMesmoFornecedor(fornecedor:{nome:string}){return fornecedor.nome == valor}


	onMount(async () => {
		listarFornecedores().then((f) => {
			fornecedores = f;
			if(valor==""){
				reset();
			}
		});
	});
</script>

<main bind:this={mainEl}>
	<label
		on:focusout={(event) => {
			let isRelated =
				event.relatedTarget instanceof Element ? mainEl.contains(event.relatedTarget) : false;
			
			if(isRelated){
				let buttonContent = event.relatedTarget instanceof HTMLButtonElement ? event.relatedTarget.innerText:'';
				if(buttonContent!=''){
					valor = buttonContent;
					
					//precisa esperar um pouco antes de dar o blur para fazer efeito.
					// timeout = 0 espera o fim do call stack
					inputEl.focus();
					setTimeout(() => {
						inputEl.blur();
					}, 0);
					
					onEdit(valor);
				}
            }
			else{
				onEdit(valor);
			}
        }}
	>
		<input
			type="text"
			bind:value={valor}
            bind:this={inputEl}
			on:input={() => {
				visualFornecedores = assignDistancesAndSort(valor);
			}}
			on:focusout={()=>{
				if(!permitirNovo && !fornecedores.find(ehMesmoFornecedor))
					valor = fornecedores[0].nome;
				onEdit(valor);
			}}
		/>
		<button type="button" class="show-suggestions" on:click={exibirTodosFornecedores}></button>
	</label>
	<ul>
		{#each visualFornecedores as fornecedor, i}
			<li><button type="button" on:click={ ()=>{}}>{fornecedor.nome}</button></li>
		{/each}
	</ul>
</main>

<style>
	main {
		position: relative;
		padding: 0;
		display: flex;
		width: 100%;
		height: var(--tema-altura-input);
		background-color: white;
		border-radius: var(--tema-border-radius);
	}
	label {
		position: relative;
		padding: 0;
		border: 2px solid black;
		border-radius: var(--tema-border-radius);
		display: flex;
		width: 100%;
	}
	main:focus-within label{
		border-radius: 0;
	}
	input {
		margin: 0;
		border: 0;
		flex-grow: 1;
		min-width: 0;
		outline: none;
		box-sizing: border-box;
		border-radius: var(--tema-border-radius);
	}
	ul {
		position: absolute;
		list-style: none;
		padding: 0;
		background-color: white;
		width: 100%;
		box-sizing: border-box;
		top: calc( var(--tema-altura-input) - 14px );
		z-index: 10;
		border-top: 0;
		max-height: 300px;
		overflow: auto;
		box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.365);
	}
	li {
		border: 1px solid black;
		overflow: hidden;
		border-width: 0px;
		max-height: 0;
		transition: max-height 0.14s, border-width 0.07s;
	}
	li button {
		width: 100%;
		background-color: transparent;
		border: none;
	}
	main:focus-within li{
		border-width: 0 1px 1px 1px;
		max-height: 100px;
	}

	main:has(label button:not(:focus)) li:nth-child(n+6) {
		border-width: 0 1px 0 1px;
		max-height: 0;
	}

	.show-suggestions {
		background-image: url('./media/down-arrow.svg');
		background-size: calc(100% - 5px) auto;
		min-width: 30px;
		background-repeat: no-repeat;
		background-position: center center;
		border: 0;
		background-color: transparent;
	}
</style>
