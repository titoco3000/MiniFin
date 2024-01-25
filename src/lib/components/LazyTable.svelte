<script lang="ts">
	import { onMount } from 'svelte';
	import arrow from '$lib/assets/arrow.svg';

	export let titulos = ['a', 'b', 'c', 'd', 'e'];
	export let infoInferior = ['soma', 'nº'];
	export let batchSize = 40;
	export let calcularMaxRows = (): number => {
		return 1000;
	};
	export let carregarValores = (
		offset: number,
		limit: number,
		sorterIndex: number,
		sorterReverse: boolean
	): string[][] => {
		let a = [];
		for (let i = offset; i < Math.min(offset + batchSize, maxRows); i++) {
			a.push(getRowAtId(i));
		}
		return a;
	};

    export function reset() {
		maxRows = calcularMaxRows();
		mainEl.scrollTop = 0;
		offset = 0;
		tbodyEl.replaceChildren();
		incluirNovosValoresNaTabela();
		firstVisibleIndex = Math.min(1, tbodyEl.childElementCount);
	}

	let tableHeaderEl: HTMLTableRowElement;
	let tbodyEl: HTMLTableSectionElement;
	let tdBlueprint: HTMLElement;
	let mainEl: HTMLElement;
	let offset = 0;
	let sorterIndex = 0;
	let sorterReverse = false;
	let firstVisibleIndex = 0;

	let maxRows: number;

	function getRowAtId(id: number) {
		return [id.toString(), id.toString(), id.toString(), id.toString(), id.toString()];
	}

	function rowToElement(row: string[]) {
		const newEL = document.createElement('tr');

		for (const val of row) {
			let td = document.createElement('td');
			td.classList.value = tdBlueprint.classList.value;
			td.innerText = val;
			newEL.appendChild(td);
		}
		return newEL;
	}

	function incluirNovosValoresNaTabela() {
		let a = carregarValores(offset, batchSize, sorterIndex, sorterReverse);
		a.forEach((row) => {
			tbodyEl.appendChild(rowToElement(row));
		});
		offset += a.length;
	}

	function onScroll(evento: Event) {
		if (evento.target) {
			let mainEl = evento.target as HTMLElement;
			let valorScroll = Math.min(
				mainEl.scrollTop / (mainEl.scrollHeight - mainEl.offsetHeight),
				1.0
			);

			if (valorScroll > 0.99) {
				incluirNovosValoresNaTabela();
			}
			//caso queira mostrar o primeiro
			//  firstVisibleIndex = Math.ceil(mainEl.scrollTop / tbodyEl.children[0].getBoundingClientRect().height);
			//caso queira mostrar do primeiro ao ultimo

			firstVisibleIndex = Math.max(
				Math.min(1, tbodyEl.childElementCount),
				Math.round(valorScroll * offset)
			);
		}
	}

	function setSorter(index: number) {
		let i = 0;
		if (index == sorterIndex) {
			sorterReverse = !sorterReverse;
		}
		tableHeaderEl.querySelectorAll('img').forEach((el) => {
			if (index != i) el.style.visibility = 'hidden';
			else el.style.visibility = 'visible';
			if (sorterReverse) el.style.transform = 'rotate(-90deg)';
			else el.style.transform = 'rotate(90deg)';
			i++;
		});
		sorterIndex = index;
		reset();
	}

	onMount(() => {
		tdBlueprint.style.display = 'none';
		reset();
	});
</script>

<main bind:this={mainEl} on:scroll={onScroll}>
	<table>
		<thead>
			<tr bind:this={tableHeaderEl}>
				{#each titulos as titulo, i}
					<th>
						<button
							on:click={() => {
								setSorter(i);
							}}
						>
							<p>
								{titulo}
							</p>
							<img src={arrow} alt="arrow" />
						</button>
					</th>
				{/each}
			</tr>
		</thead>
		<td bind:this={tdBlueprint}></td>
		<tbody bind:this={tbodyEl}> </tbody>
		<tfoot>
			<tr>
				{#each infoInferior as item}
					<td>
						{item}
					</td>
				{/each}
				<td>
					{firstVisibleIndex}/{maxRows}
				</td>
			</tr>
		</tfoot>
	</table>
</main>

<style>
	main {
		width: 600px;
		height: 500px;
		margin: 20px;
		overflow: auto;
		padding: 0;
		border: 1px dashed black;
	}
	table {
		table-layout: fixed;
		width: 100%;
		height: 100%;
		text-align: left;
		position: relative;
		border-collapse: separate;
		border-spacing: 0;
	}
	th,
	td {
		border: 1px solid black;
		text-wrap: nowrap;
		overflow: hidden;
	}
	th {
		position: sticky;
		top: 0; /* Don't forget this, required for the stickiness */
		background-color: var(--cor-tema-fraca);
		padding: 0;
	}
	tfoot td {
		position: sticky;
		bottom: 0; /* Don't forget this, required for the stickiness */
		background-color: var(--cor-tema-fraca);
	}
	thead button {
		display: flex;
		width: 100%;
		height: 100%;
		margin: 0;
		padding: 0;
		border: none;
		background-color: transparent;
	}
	thead button > p {
		margin: 0;
		margin: auto;
		min-width: 0;
		flex-grow: 1;
		text-wrap: nowrap;
		overflow: hidden;
	}
	thead button > img {
		height: 20px;
		flex: 0 0 20px;
		padding: 5px;
		transform: rotate(90deg);
		visibility: hidden;
	}
	thead th:first-child button > img {
		visibility: visible;
	}
</style>
