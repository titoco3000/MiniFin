<script lang="ts">
	import { onMount } from 'svelte';

	export function obterValor() {
		return parseInt(inputEl.value.replace(/[^0-9\-]/g, ''));
	}

	export function reset() {
		inputEl.value = '0' + separadorCentavos + '00';
	}

	export let separadorCentavos = ',';

	let separadorMilhares: string;

	let inputEl: HTMLInputElement;

	function onInput(e: Event) {
		if (e.target) {
			let s = inputEl.value.replace(/[^0-9.,]/g, '');

			let caretPos = inputEl.selectionStart;
			caretPos ??= s.length;

			caretPos += s.length - inputEl.value.length;

			inputEl.value = s;

			inputEl.setSelectionRange(caretPos, caretPos);
		}
	}

	function onBlur(e: Event) {
		if (e.target) {
			let semSepMil = inputEl.value.replaceAll(separadorMilhares, '');
			let posSepDecimal = semSepMil.lastIndexOf(',');

			let parteInteira = semSepMil;
			let parteDecimal = '00';

			//se tem separador decimal
			if (posSepDecimal >= 0) {
				parteInteira = semSepMil.slice(0, posSepDecimal).replaceAll(separadorCentavos, '');
				if (parteInteira == '') parteInteira = '0';
				parteDecimal = semSepMil.slice(posSepDecimal + 1, semSepMil.length);
				if (parteDecimal == '') {
					parteDecimal = '00';
				}
			}

			//remove 0s do inicio
			parteInteira = '' + parseInt(parteInteira);
			//adiciona separadores de milhares
			parteInteira = parteInteira.replace(/\B(?=(\d{3})+(?!\d))/g, separadorMilhares);

			inputEl.value = parteInteira + ',' + (parteDecimal + '0').slice(0, 2);
		}
	}

	onMount(async () => {
		if (separadorCentavos == ',') separadorMilhares = '.';
		else separadorMilhares = ',';
		reset();
	});
</script>

<div>
	<input type="text" value="0,00" bind:this={inputEl} on:input={onInput} on:blur={onBlur} />
</div>

<style>
	div {
		position: relative;
		padding: 0;
		display: flex;
		width: 100%;
		/* box-sizing: border-box; */
	}
	input {
		margin: 0;
		border: 0;
		flex-grow: 1;
		min-width: 0;
		outline: none;
		border: 1px solid black;
	}
</style>
