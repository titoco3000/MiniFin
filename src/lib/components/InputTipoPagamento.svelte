<script lang="ts">
	import { onMount } from 'svelte';
	import Dropdown from './Dropdown.svelte';
	import { listarPagamentos } from '$lib/armazenamento';

	export function obterValor() {
		return dpdEl.obterValor();
	}
	export function reset() {
		dpdEl.reset();
	}

	let dpdEl: any;
	onMount(async () => {
		reset();
		listarPagamentos().then((r) => {
			dpdEl.setDados(r.map(a => a.nome));
		});
	});
</script>

<Dropdown bind:this={dpdEl} dados={[['a','b']]}/>

<style>
</style>
