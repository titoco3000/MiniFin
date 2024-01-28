<script lang="ts">
	import { onMount } from 'svelte';

	export let valor = new Date().toISOString().split('T')[0];
	export let onChange = () => {};
	export let placeholder = new Date().toISOString().split('T')[0];

	let valueVisivel = valor;

	export function reset() {
		valor = placeholder;
		atualizarVisivel();
	}
	export function obterValor(){
		return valor;
	}

	function formatar(data: string) {
		let partes = data.split('-');
		return partes[2] + '/' + partes[1] + '/' + partes[0];
	}
	function atualizarVisivel() {
		valueVisivel = formatar(valor);
	}

	onMount(() => {
		atualizarVisivel();
	});
</script>

<main>
	<label>
		{valueVisivel}
		<input
			type="date"
			name="data"
			bind:value={valor}
			on:change={() => {
				atualizarVisivel();
				onChange();
			}}
			on:focus={(e) => {
				e.currentTarget.showPicker();
			}}
		/>
	</label>
</main>

<style>
	main {
		width: 100%;
		height: var(--tema-altura-input);
		position: relative;
	}
	input {
		height: 100%;
		width: 100%;
		padding: 0;
		margin: 0;
        position: absolute;
        z-index: -10;
	}
	label {
		font-size: 16px;
		display: flex;
		justify-content: space-around;
		align-items: center;
		height: 100%;
		width: 100%;
		position: absolute;
		top: 0;
		left: 0;
		background-color: var(--cor-tema-fundo-2);
		border: 2px solid var(--cor-tema-detalhes);
		border-radius: var(--tema-border-radius);
	}
</style>
