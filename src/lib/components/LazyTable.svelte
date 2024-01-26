<script lang="ts">
	import { onMount } from 'svelte';
	import arrow from '$lib/assets/arrow.svg';

	export let titulos = ['a', 'b', 'c', 'd', 'e'];
	export let legendaInferior = 'Total';
	export let valorInferior = '';
	export let batchSize = 40;
	export let calcularMaxRows = async (): Promise<number> => {
		return 1000;
	};
	export let carregarValores = async (
		offset: number,
		limit: number,
		sorterIndex: number,
		sorterReverse: boolean
	): Promise<string[][]> => {
		let a = [];
		for (let i = offset; i < Math.min(offset + batchSize, maxRows); i++) {
			a.push(getRowAtId(i));
		}
		return a;
	};

	export function reset() {
		calcularMaxRows().then((v) => {
			maxRows = v;
		});
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
		carregarValores(offset, batchSize, sorterIndex, sorterReverse).then((a) => {
			a.forEach((row) => {
				tbodyEl.appendChild(rowToElement(row));
			});
			offset += a.length;
            
            if(firstVisibleIndex ==0)
                firstVisibleIndex = 1;
		});
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
			if (isNaN(firstVisibleIndex)) firstVisibleIndex = 1;
		}
	}

	function setSorter(index: number) {
		let i = 0;
		if (index == sorterIndex) {
			sorterReverse = !sorterReverse;
		}
		tableHeaderEl.querySelectorAll('img').forEach((el) => {
			if (index != i) el.style.display = 'none';
			else el.style.display = 'inline';
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
				<td colspan={(titulos.length-2)-Math.floor((titulos.length-2)/2)} style="text-align: right;">{legendaInferior}</td>
				<td colspan={Math.floor((titulos.length-2)/2)}>{valorInferior}</td>
				<td colspan="2" style="text-align: right;">
					{firstVisibleIndex}/{maxRows}
				</td>
			</tr>
		</tfoot>
	</table>
</main>

<style>
	main {
		width: 100%;
		height: 100%;
		overflow: auto;
		padding: 0;
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
		display: none;
	}
	thead th:first-child button > img {
		display: inline;
	}
</style>
