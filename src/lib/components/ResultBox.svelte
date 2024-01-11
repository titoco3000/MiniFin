<script lang="ts">
	let valor = '';
	let pEl: HTMLElement;
	let holderEl: HTMLElement;
	let timeoutID: number | null = null;
	export function mensagem(msg: string, tipo: string = 'bom', tempo: number = 100000) {
		valor = msg;

		if (timeoutID) {
			console.log('clearing timeout');

			clearTimeout(timeoutID);
		}

		timeoutID = setTimeout(() => {
			console.log('invisivel');
			holderEl.style.opacity = '0%';
			holderEl.style.transition = 'opacity 1s';
			timeoutID = null;
		}, tempo);
        
		if (tipo[0] == 'b') pEl.style.color = 'var(--b)';
		else if (tipo[0] == 'r') pEl.style.color = 'var(--r)';
		else pEl.style.color = 'var(--n)';
		holderEl.style.opacity = '100%';
        holderEl.style.transition = 'opacity 0s';
	}
</script>

<div bind:this={holderEl}>
	<p bind:this={pEl}>{valor}</p>
</div>

<style>
	div {
		width: 100%;
		background-color: #d9d9d9;
        border-radius: 5px;
		box-shadow: 1px 1px 4px rgba(0, 0, 0, 0.5);
		overflow: hidden;
        opacity: 0%;
	}
	p {
        padding: 10px;
        margin: 0;
		--b: blue; /*boa*/
		--r: red; /*ruim*/
		--n: black; /*neutra*/
		white-space:pre;  
	}
</style>
