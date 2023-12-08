<script lang="ts">
	import { onMount } from 'svelte';
	import Dropdown from './Dropdown.svelte';
	import { listarSetores } from '$lib/armazenamento';

	export function obterValor() {
		return dpdEl.obterValor();
	}
	export function reset() {
		dpdEl.reset();
	}
	export let valor: string[] = ['A'];
	export let onEdit = (v:string[])=>{};

	let dpdEl: any;
	let dados: any = [['A',['1','2']],['B',['1','2']]];
	onMount(async () => {
		reset();
		listarSetores().then((setores) => {
			let empresas:any = [];
			let setoresPorEmpresa:string[][] = [];
			setores.forEach(v=>{
				let index = empresas.indexOf(v.empresa);
				if(index<0){
					empresas.push(v.empresa);
					setoresPorEmpresa.push([v.nome]);
				}
				else
					setoresPorEmpresa[index].push(v.nome);
			});
			for (let i = 0; i < empresas.length; i++) {
				empresas[i] = [empresas[i], setoresPorEmpresa[i]];
			}
			dpdEl.setDados(empresas);
		});
	});
</script>

<Dropdown onEdit={onEdit} bind:this={dpdEl} bind:value={valor} dados={[['A',['1','2']]]} visualTreatment={v=>{
	return v.join(': ');
}}/>

<style>
</style>
