<script lang="ts">
	
	import Formulario from '$lib/components/Formulario.svelte';
	import TabelaGastos from '$lib/components/TabelaGastos.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import { onMount } from 'svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';

	let navEl: HTMLElement;
	let formSecEl: HTMLElement;
	let vizSecEl: HTMLElement;
	let ferramentasSecEl: HTMLElement;

	function revelarSection(sec: HTMLElement) {
		formSecEl.style.display = 'none';
		vizSecEl.style.display = 'none';
		ferramentasSecEl.style.display = 'none';

		sec.style.display = 'block';
	}
	function formSubmit(e: { preventDefault: () => void; }){
		e.preventDefault();
		console.log('submit');
		
	}

	function changeHeaderColors(src:EventTarget|null){
		if(src){
			for(let child of navEl.children){
				(child as HTMLElement).style.backgroundColor = "var(--cor-tema-fraca)";
			}
			(src as HTMLElement).style.backgroundColor = "var(--cor-tema-forte)";

		}
		
	}
	onMount(() => {
		revelarSection(formSecEl);
	});
</script>
<main>
	<nav bind:this={navEl}>
		<button
			on:click={(e) => {
				changeHeaderColors(e.target);
				revelarSection(formSecEl);
			}}>Formulário</button
		>
		<button
			on:click={(e) => {
				changeHeaderColors(e.target);
				revelarSection(vizSecEl);
			}}>Visualizar</button
		>
		<button
			on:click={(e) => {
				changeHeaderColors(e.target);
				revelarSection(ferramentasSecEl);
			}}>Ferramentas</button
		>
		<button on:click={() => {
			window.location.reload()
		}}>Reload</button
		>

	</nav>
	<section bind:this={formSecEl}>
		<Formulario />
	</section>
	<section bind:this={vizSecEl}>
		<TabelaGastos />
	</section>
	<section bind:this={ferramentasSecEl}>
		<div class="temp">
			<Dropdown dados={[['A', [['1','N'], '2', '3']], ['B'], ['C', ['1', '2', ['3D', ['1', '2']]]]]} />
		</div>
	</section>
</main>

<style> 
	main {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
	}
	nav {
		background-color: var(--cor-tema-fundo-1);	
		margin: 0;
		padding-left: 10px;
	}
	nav button {
		font-size: large;
		margin: 10px;
		border-width: 0;
		border-radius: 5px;
		background-color: var(--cor-tema-fraca);
		padding: 10px;
	}
	section {
		display: none;
		flex: 1;
		overflow: auto;
		padding: 20px;
	}
	.temp{
		width: 600px;
		margin: auto;
		margin-top: 20px;
	}
</style>
