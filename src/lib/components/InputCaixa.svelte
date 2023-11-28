<script lang="ts">
	import { onMount } from 'svelte';
	import Dropdown from './Dropdown.svelte';
	import { listarEmpresas } from '$lib/armazenamento';

	export function obterValor() {
		return dpdEl.obterValor();
	}
	export function reset() {
		dpdEl.reset();
	}

	let dpdEl: any;
	onMount(async () => {
		reset();
		listarEmpresas().then((r) => {
			dpdEl.setDados(r.map(a => a.nome));
		});
	});
</script>

<Dropdown bind:this={dpdEl} dados={[['a','b']]}/>

<style>
</style>
