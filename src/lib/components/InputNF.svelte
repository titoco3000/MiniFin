<script lang="ts">
	import { onMount } from 'svelte';
	import { obterDigitosNF } from '$lib/utils';
	export function obterValor() {
		return parseInt(inputEl.value.replace(/[^0-9\-]/g, ''));
	}
	export function reset(){
		let v = '';
		for (let i = 0; i < digitos; i++) v += '0';
		inputEl.value = v;
	}

	let digitos = 9;

	let inputEl: HTMLInputElement;

	function occurrences(str_: string, subStr: string) {
		let occurence_count = 0;
		let pos = -subStr.length;
		while ((pos = str_.indexOf(subStr, pos + subStr.length)) > -1) {
			occurence_count++;
		}
		return occurence_count;
	}

	function onInput(e: Event) {
		if (e.target) {
			let s = inputEl.value.replace(/[^0-9\-]/g, '');

			let digitosAtuais = s.length - occurrences(s, '-');

			let caretPos = inputEl.selectionStart;
			caretPos ??= s.length;

			caretPos += s.length - inputEl.value.length;

			while (digitosAtuais < digitos) {
				digitosAtuais++;
				s += '0';
			}
			while (digitosAtuais > digitos) {
				if (s[s.length - 1] != '-') digitosAtuais--;
				s = s.slice(1);
			}

			// valor = s;
			inputEl.value = s;

			inputEl.setSelectionRange(caretPos, caretPos);
		}
	}

	onMount(async () => {
		digitos = obterDigitosNF();
		reset();
	});
</script>

<div>
	<input type="text" bind:this={inputEl} on:input={onInput} />
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
		border: 2px solid var(--cor-tema-detalhes);
		border-radius: var(--tema-border-radius);
		font-size: 16px;
		height: var(--tema-altura-input);
		box-sizing: border-box;
	}
</style>
