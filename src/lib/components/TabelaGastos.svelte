<script lang="ts">
	import InputFornecedor from '$lib/components/InputFornecedor.svelte';
	import InputSetor from '$lib/components/InputSetor.svelte';
	import InputCaixa from '$lib/components/InputCaixa.svelte';
	import InputEmpresa from './InputEmpresa.svelte';
	import InputTipoPagamento from './InputTipoPagamento.svelte';

	function selecionarFiltro(e:any) {
		let el = e.srcElement.parentNode.querySelector('.input-holder');
		if (e.currentTarget.checked){
            el.style.setProperty('--opacity', '0');
            el.style.setProperty('--blur-amount', '0px');
            el.style.setProperty('--pointer-events', 'none');
        }
        else{
            el.style.setProperty('--opacity', '0.5');
            el.style.setProperty('--blur-amount', '1px');
            el.style.setProperty('--pointer-events', 'all');
        }
	}
</script>

<main>
	<div class="filtro">
		<h2>Filtros</h2>
		<button>Remover Filtros</button>
		<div>
			<h3>Data inicial</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<input type="date" />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Data final</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<input type="date" />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Fornecedor</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputFornecedor />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Empresa</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputEmpresa />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Setor</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputSetor />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Tipo de pagamento</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputTipoPagamento />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
		<div>
			<h3>Caixa de entrada</h3>
			<div class="controls-holder">
				<div class="input-holder">
					<InputCaixa />
				</div>
				<input type="checkbox" on:change={selecionarFiltro} />
			</div>
		</div>
	</div>
	<div class="table-holder">
		<table>
			<tr>
				<th>Dada</th>
				<th>Fornecedor</th>
				<th>Empresa</th>
				<th>Setor</th>
				<th>Valor</th>
				<th>Pagamento</th>
				<th>NF</th>
				<th>Caixa</th>
				<th>Observações</th>
			</tr>
			<tr>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
				<td>?</td>
			</tr>
		</table>
	</div>
</main>

<style>
	main {
		display: flex;
		width: 100%;
		height: 100%;
	}
	.table-holder {
		flex-grow: 1;
	}
	th,
	td {
		background-color: white;
	}
	.filtro {
		width: 150px;
		background-color: white;
	}
	.filtro h3 {
		margin: 0;
		flex-grow: 1;
		width: 100%;
	}
	.filtro > div {
		display: flex;
		flex-direction: column;
		display: inline;
	}
	.controls-holder {
		display: flex;
		justify-content: space-between;
	}
	.input-holder {
		flex-grow: 1;
		flex-shrink: 1;
		max-width: calc(100% - 26px);
		display: flex;
		position: relative;
        --blur-amount: 1px;
        --opacity: 0.5;
        --pointer-events:all
	}
	.input-holder::after {
		content: '';
		position: absolute;
		left: 0;
		right: 0;
		width: 100%;
		height: 100%;
		backdrop-filter: blur(var(--blur-amount));
		background-color: rgba(255, 255, 255, var(--opacity));
        pointer-events: var(--pointer-events);
	}
</style>
