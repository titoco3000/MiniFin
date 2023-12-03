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
	export let valor:string[]=[''];
	export let onEdit = (v:string[])=>{};

	let dpdEl: any;
	onMount(async () => {
		reset();
		listarEmpresas().then((r) => {
			dpdEl.setDados(r.map(a => a.nome));
		});
	});
</script>

<Dropdown onEdit={onEdit} bind:this={dpdEl} bind:value={valor} dados={[['a','b']]}/>

<style>
</style>
