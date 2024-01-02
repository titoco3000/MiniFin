<script lang="ts">
	export let dados: any = [
		'main',
		[
			['item 1', []],
			['title2', [['item 1',[]], ['item 2',[]]]]
		]
	];

	export let onBubbleUp = (v:string[])=>{};

	function childBubbleUpFn(v:string[]){
		onBubbleUp([dados[0]].concat(v));
	};
</script>

<main>
	<button on:click={()=>{
		if(dados[1].length == 0){
			onBubbleUp([dados[0]]);
		}
		
	}}>
		{dados[0]}
	</button>
	<ul>
		{#each dados[1] || [] as item}
			<li>
				<svelte:self dados={item} onBubbleUp={childBubbleUpFn}/>
			</li>
		{/each}
	</ul>
</main>

<style>
	* {
		padding: 0;
		margin: 0;
		box-sizing: border-box;
	}
	main {
		width: 100%;
        height: 100%;
		position: relative;
	}
	button {
		border: 1px solid black;
		width: 100%;
        height: 100%;
        display: block;
		text-wrap: nowrap;
	}
	ul {
		list-style: none;
		position: absolute;
		width: 100%;
		top: 0;
		left: calc(100% - 0.5px);
	}
	li:not(:first-child) {
		border-top-width: 0;
	}
</style>
