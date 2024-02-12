<script lang="ts">
	import { onMount } from 'svelte';
	import FileChooser from '$lib/components/FileChooser.svelte';
	import Console from '$lib/components/Console.svelte';
	import TopHeader from '$lib/components/TopHeader.svelte';
	import { importarCSVs, definirLocalDB } from '$lib/armazenamento';
	
	let navEl: HTMLElement;
	let modosHolderEl: HTMLElement;
	let consoleCSV:HTMLElement;

	let pastaDB: string;

	let csvPaths = ['', ''];
	function abrirSection(evento: Event) {
		let buttons = [...navEl.children];
		let index = buttons.indexOf(evento.target as HTMLElement);

		(modosHolderEl.children[index] as HTMLElement).style.display = 'block';
	}

	function enviarFormCSVs() {
		ativarConsole(consoleCSV);
		definirLocalDB(pastaDB).then((r:any) => {
			console.log("r:",r);
			console.log(r.Ok===undefined,r.Err===undefined);
			
			if (r.Ok !==undefined) {
				importarCSVs(csvPaths[0], csvPaths[1]).then((r) => {
					if (r.Ok !==undefined) {
						window.location.replace("form");
					} else {
						console.log('Algo de errado ocorreu: ', r);
						alert('Algo de errado ocorreu ao enviar CSVs');
					}
				});
			} else {
				console.log('Algo de errado ocorreu: ', r);
				alert('Algo de errado ocorreu ao enviar local do bd');
			}
		});
	}

	function ativarConsole(c:HTMLElement){
		c.style.display = "flex";
	}

	onMount(() => {});
</script>

<main>
	<TopHeader options={[]}/>
	<div id="content">
		<div id="intro">
			<h1>MiniFin</h1>
			<p>Bem vindo a <em>MiniFin</em>, o sistema de gestão financeira para múltiplas empresas!</p>
			<p>
				Se você está vendo essa página, é porque é a primeira vez abrindo esse programa, nesse caso
				vá para <em>Configuração inicial</em>. Ou, se você já tem dados mas aconteceu algum problema
				com o seu arquivo de configuração, vá para <em>Localizar banco de dados</em>. Se você está
				migrando de algum sistema compatível, escolha uma das opções para importar dados.
			</p>
		</div>
		<nav bind:this={navEl}>
			<button on:click={abrirSection}>Configuração inicial</button>
			<button on:click={abrirSection}>Localizar banco de dados</button>
			<button on:click={abrirSection}>Importar CSVs</button>
		</nav>
	</div>
	<div bind:this={modosHolderEl}>
		<form id="inicial">
			<FileChooser pastas={true} bind:value={pastaDB} />
		</form>
		<form id="localizar"></form>
		<form id="CSVs" on:submit={enviarFormCSVs}>
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label>
				<p>Primeiro, escolha onde deverão ser salvos os dados:</p>
				<FileChooser pastas={true} bind:value={pastaDB} />
			</label>
			<p>(Será criada uma pasta <em>/minifin</em> no local selecionado)</p>

			<p>Selecione os arquivos para serem importados:</p>
			<div class="CSV-choosers">
				<div>
					<FileChooser placeholder={'Selecione fornecedores.csv'} bind:value={csvPaths[0]} />
				</div>
				<div>
					<FileChooser placeholder={'Selecione compras.csv'} bind:value={csvPaths[1]} />
				</div>
			</div>
			{#if csvPaths[0] != '' && csvPaths[1] != '' && pastaDB != ''}
				<input type="submit" />
			{/if}
			<div class="page-cover" bind:this={consoleCSV}>
				<div class="console-container">
					<Console/>
				</div>
			</div>
		</form>
	</div>
</main>

<style>
	.page-cover{
		position: fixed;
		left: 0;
		top: 30px;
		height: calc(100vh - 30px);
		width: 100vw;
		background-color: rgba(128, 128, 128, 0.14);
		backdrop-filter: blur(5px);
		display: none;
		justify-content: center;
		align-items: center;
	}
	.console-container{
		width: 100%;
		height: 100%;
		max-width: 600px;
		max-height: 400px;
	}
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100vh;
	}
	h1 {
		font-size: 100px;
		color: var(--cor-tema-forte);
		background-color: var(--cor-tema-fundo-2);
		margin: 0;
		padding: 30px;
		-webkit-text-stroke: 2px black; /* width and color */
	}
	form {
		display: none;
		position: absolute;
		top: 30px;
		left: 0;
		height: calc(100% - 30px);
		width: 100%;
		background-color: var(--cor-tema-fundo-2);
		padding: 20px;
	}
	#content {
		display: flex;
		flex-grow: 1;
	}
	#intro {
		padding: 30px;
		max-width: 60%;
	}
	nav {
		padding-top: 200px;
		display: flex;
		flex-direction: column;
		background-color: var(--cor-tema-fundo-1);
		flex-grow: 1;
	}
	nav > button {
		background-color: var(--cor-tema-fraca);
		border: none;
		border-radius: var(--tema-border-radius);
		margin: 10px;
		min-height: var(--tema-altura-input);
		font-size: 16px;
	}
	.CSV-choosers {
		display: flex;
	}
	.CSV-choosers > div {
		margin-right: 10px;
	}
</style>
