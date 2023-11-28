<script lang="ts">
	import { onMount } from 'svelte';

	export let dados: any;
	export let bubbleUp:any;
	// export let bubbleUp: (string[])=>any;

	let btnEl: HTMLElement;

	function isString(x: any): boolean {
		return Object.prototype.toString.call(x) === '[object String]';
	}

	function onBubbleUp(v: string[]) {
		v.unshift(dados[0]);
		bubbleUp(v);
	}

	onMount(async () => {
		if (isString(dados)) {
			btnEl.style.cursor = 'pointer';
			btnEl.classList.add('adb');//active dropdown button
		}
		
	});
</script>

<li>
	{#if isString(dados)}
	<button
		bind:this={btnEl}
		on:click={() => {
			bubbleUp([dados]);
		}}><p>{dados}</p></button
	>
	{:else}
	<button
		bind:this={btnEl}
		on:click={() => {
			if (!dados[1]) {
				bubbleUp([dados[0]]);
			}
		}}><p>{dados[0]}</p></button
	>
	<ul>
		{#each dados[1] || [] as dado}
			<svelte:self dados={dado} bubbleUp={onBubbleUp} />
		{/each}
	</ul>
	{/if}
</li>

