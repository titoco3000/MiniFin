<script lang="ts">
	import { onMount } from 'svelte';
	import arrow from '$lib/assets/arrow.svg';

	export let titulos: string[] = [];
	export let legendaInferior = 'Total';
	export let valorInferior = '';
	export let batchSize = 70;
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
		console.log('reset table');

		mainEl.scrollTop = 0;
		offset = 0;
		tbodyEl.replaceChildren();
		incluirNovosValoresNaTabela();

		calcularMaxRows().then((v) => {
			maxRows = v;
			firstVisibleIndex = sorterReverse ? maxRows : Math.min(1, tbodyEl.childElementCount);
		});
		
	}

	export let exportar = (sorterIndex: number,
		sorterReverse: boolean) => { };

	let tableHeaderEl: HTMLTableRowElement;
	let tbodyEl: HTMLTableSectionElement;
	let tdBlueprint: HTMLElement;
	let mainEl: HTMLElement;
	let offset = 0;
	let sorterIndex = 0;
	let sorterReverse = true;
	let firstVisibleIndex = 0;

	let maxRows: number = 0;

	function getRowAtId(id: number) {
		return [id.toString(), id.toString(), id.toString(), id.toString(), id.toString()];
	}

	function rowToElement(row: string[]) {
		const newEL = document.createElement('tr');

		for (const val of row) {
			let td = document.createElement('td');
			td.onmouseenter = mouseEnter;
			td.onmouseleave = mouseLeave;
			td.classList.value = tdBlueprint.classList.value;
			td.innerText = val;
			newEL.appendChild(td);
		}
		return newEL;
	}

	let timerToolTip: number;
	let tooltipEl: HTMLElement;

	function getAbsoluteOffsetLeft(el: HTMLElement) {
		let offset = 0;
		let currentElement: HTMLElement | null = el;

		while (currentElement !== null) {
			offset += currentElement.offsetLeft;
			offset -= currentElement.scrollLeft;
			currentElement = currentElement.offsetParent as HTMLElement;
		}

		return offset;
	}

	function mouseEnter(e: MouseEvent) {
		timerToolTip = setTimeout(() => {
			if (e.target) {
				let target = e.target as HTMLElement;
				if (target.innerText !== '') {
					tooltipEl.innerText = target.innerText;

					tooltipEl.style.right =
						window.innerWidth -
						getAbsoluteOffsetLeft(target) -
						target.getBoundingClientRect().width / 2 +
						'px';
					tooltipEl.style.top =
						8 + target.getBoundingClientRect().top + target.getBoundingClientRect().height + 'px';

					tooltipEl.style.opacity = '100%';
				}
			}
		}, 1000);
	}
	function mouseLeave(e: Event) {
		clearTimeout(timerToolTip);
		tooltipEl.style.opacity = '0';
	}

	function incluirNovosValoresNaTabela() {
		carregarValores(offset, batchSize, sorterIndex, sorterReverse).then((a) => {
			a.forEach((row) => {
				tbodyEl.appendChild(rowToElement(row));
			});
			offset += a.length;
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

			
			firstVisibleIndex = sorterReverse?
				maxRows - Math.round(valorScroll * Math.max(0, tbodyEl.childElementCount-1)):
				Math.round(valorScroll * Math.max(0, tbodyEl.childElementCount-1))+1
			;
			if (isNaN(firstVisibleIndex)) firstVisibleIndex = 0;
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
				<td
					colspan={titulos.length - 2 - Math.floor((titulos.length - 2) / 2)}
					style="text-align: right;"><div id="left-footer">
						<button on:click={()=>exportar(sorterIndex, sorterReverse)}>
							<div id="spreadsheet-container">
								<div id="spreadsheet-icon">
									<table>
										<tr><td></td><td></td></tr>
										<tr><td></td><td></td></tr>
										<tr><td></td><td></td></tr>
									</table>
								</div>
								
							</div>
							<span>Exportar</span>
						</button>
						<span>
							{legendaInferior}
						</span>
					</div></td
				>
				<td colspan={Math.floor((titulos.length - 2) / 2)}>{valorInferior}</td>
				<td colspan="2" style="text-align: right;">
					{firstVisibleIndex}/{maxRows}
				</td>
			</tr>
		</tfoot>
	</table>
</main>
<p id="tooltip" bind:this={tooltipEl}></p>

<style>
	#left-footer{
		display: flex;
		justify-content: space-between;
	}
	#left-footer button{
		padding: 0;
		border: none;
		display: flex;
		align-items: center;
		background-color: transparent;
	}
	#spreadsheet-container{
		height: 20px;
		width: 20px;
		display: inline-block;
		position: relative;
	}
	#spreadsheet-icon{
		background-color: #2d9b2d;
		height: 24px;
		width: 20px;
		position: absolute;
		border-radius: 2px;
		top: -2px;
		left: 0;
		padding: 8px 5px;
		transform: rotate(0);
	}
	button:hover #spreadsheet-icon{
		animation: wave 0.8s forwards;
	}
	#spreadsheet-icon::after{
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		background-color: var(--cor-tema-fraca);
		width: 6px;
		height: 6px;
		border-radius: 0 0 0 2px;
		background-image:
			linear-gradient(to bottom left, transparent 50%, rgb(191, 205, 218) 0),
			linear-gradient(to top right, rgb(191, 205, 218) 50%, transparent 0);
	}
	#spreadsheet-icon table{
		height: 10px;
		border-collapse: collapse;
	}
	#spreadsheet-icon td{
		padding: 0;
		height: 0px;
		width: 1px;
		background-color: transparent;
		border: 1px solid white;
	}
	#left-footer span{
		display: flex;
		align-items: center;
	}
	#left-footer button span{
		width: 0px;
		transition: width 0.5s, color 0.5s;
		justify-content: end;
		overflow: hidden;
		color: #2d9b2d;
	}
	#left-footer button:hover span{
		width: 55px;
		color: black;
	}
	@keyframes wave{
		0%{
			transform: rotate(0);
		}
		25%{
			transform: rotate(5deg);
		}
		70%{
			transform: rotate(-3deg);
		}
		100%{
			transform: rotate(0);
		}
	}


	#tooltip {
		background-color: var(--cor-tema-fraca);
		border: 2px solid black;
		opacity: 0;
		position: absolute;
		top: 0;
		margin: 0;
		padding: 10px;
		border-radius: var(--tema-border-radius);
		border-top-right-radius: 3px;
		transition: opacity 0.2s;
		user-select: none;
		pointer-events: none;
	}
	#tooltip::after {
		position: absolute;
		right: 2px;
		top: -7px;
		height: 10px;
		width: 10px;
		content: '';
		transform: rotate(45deg);
		background-color: var(--cor-tema-fraca);
		border: 2px solid black;
		border-width: 2px 0 0 2px;
	}
	main {
		width: 100%;
		height: 100%;
		overflow-y: scroll;
		overflow-x: visible;
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
		border: 1px solid rgb(138, 138, 138);
		text-wrap: nowrap;
		overflow: hidden;
	}
	@supports (not (text-wrap: nowrap)) {
		th,
		td {
			white-space: nowrap;
		}
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
		padding: 5px;
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
		transform: rotate(-90deg);
		display: none;
	}
	thead th:first-child button > img {
		display: inline;
	}

	/* Aqui já desisti de toda generalidade */
	th:nth-child(1) {
		width: 75px;
	}
	th:nth-child(3) {
		width: 7%;
	}
	th:nth-child(5) {
		width: 75px;
	}
	th:nth-child(6) {
		width: 8%;
	}
	th:nth-child(7) {
		width: 75px;
	}
	th:nth-child(8) {
		width: 10%;
	}
	th:nth-child(9) {
		width: 20%;
	}
</style>
