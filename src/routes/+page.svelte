<script lang="ts">
	
	import Formulario from '$lib/components/Formulario.svelte';
	import TabelaGastos from '$lib/components/TabelaGastos.svelte';
	import { onMount } from 'svelte';

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
	onMount(() => {
		revelarSection(formSecEl);
	});
</script>
<main>
	<nav>
		<button
			on:click={() => {
				revelarSection(formSecEl);
			}}>Formulário</button
		>
		<button
			on:click={() => {
				revelarSection(vizSecEl);
			}}>Visualizar</button
		>
		<button
			on:click={() => {
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
		background-color: rgb(96, 96, 96);
		margin: 0;
	}
	nav button {
		font-size: large;
	}
	section {
		display: none;
		flex: 1;
		overflow: auto;
		padding: 10px;
	}
</style>
