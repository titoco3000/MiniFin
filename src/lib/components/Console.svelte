<script lang="ts">
	import { extrairDadosTerminal } from '$lib/armazenamento';
	import { onMount } from 'svelte';
	let maxLines = 32;
	let mainEl: HTMLElement;
	let blueprint: HTMLElement;

	let goodColor = 'green';
	let alertColor = 'yellow';
	let badColor = 'red';

	let interval = 500;

	onMount(()=>{
		new IntersectionObserver((entries, observer) => {
			entries.forEach((entry) => {
				if (entry.intersectionRatio > 0) {
					setInterval(() => {
				extrairDadosTerminal().then((d: string[][]) => {
					console.log(d);

					d.forEach((e) => {
						let shouldScrollDown =
							Math.min(mainEl.scrollTop / (mainEl.scrollHeight - mainEl.offsetHeight), 1.0) ==
								1.0 || mainEl.scrollHeight == mainEl.offsetHeight - 4;

						console.log(mainEl.scrollHeight, mainEl.offsetHeight);
						let p = blueprint.cloneNode() as HTMLElement;
						p.style.display = 'block';
						p.innerText = e[0];
						if (e[1] == 'Good') p.style.color = goodColor;
						if (e[1] == 'Alert') p.style.color = alertColor;
						if (e[1] == 'Bad') p.style.color = badColor;
						mainEl.appendChild(p);

						if (mainEl.childElementCount > maxLines) {
							setTimeout(() => {
								if (mainEl.firstElementChild) mainEl.removeChild(mainEl.firstElementChild);
							}, interval / 2);
						}
						//se estava no máx de scroll ou se ainda não tinha tamanho para scroll
						if (shouldScrollDown) {
							mainEl.scrollTo(0, mainEl.scrollHeight);
						}
					});
				});
			}, interval);
					
					observer.disconnect();
				}
			});
		}).observe(mainEl);
	});
</script>

<main bind:this={mainEl}></main>
<p id="blueprint" bind:this={blueprint}></p>

<style>
	main {
		background-color: black;
		width: 100%;
		height: 100%;
		padding: 10px;
		border-radius: var(--tema-border-radius);
		border: 2px solid gray;
		overflow: auto;
	}
	p {
		color: white;
		font-family: monospace;
		margin: 10px 0 0 0;
	}
	p:first-child {
		margin-top: 0;
	}
</style>
