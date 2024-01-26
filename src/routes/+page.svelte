<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri'
	import LazyTable from '$lib/components/LazyTable.svelte';

	let loadingEl: HTMLCanvasElement;

	function drawArc() {
		let context = loadingEl.getContext('2d');
		if (context) {
			let x = loadingEl.width / 2;
			let y = loadingEl.height / 2;
			let radius = 75;
			let startAngle = 0.8 * Math.PI;
			let endAngle = 1.9 * Math.PI;
			let counterClockwise = false;

			context.beginPath();
			context.arc(x, y, radius, startAngle, endAngle, counterClockwise);
			context.lineWidth = 15;

			// line color
			context.strokeStyle = getComputedStyle(context.canvas).getPropertyValue('--cor-tema-forte');
			context.stroke();
		}
	}
	onMount(() => {
		drawArc();

		invoke("checar_tipo_de_janela").then(resultado=>{
			if(resultado == "instalacao")
				window.location.replace("instalacao");
			else
				window.location.replace("form");
			
		})
	});
</script>

<main>
	<LazyTable />
	<div class="splash">
		<h1>Raja</h1>
		<canvas bind:this={loadingEl} width="200" height="200"></canvas>
	</div>
</main>

<style>
	main {
		height: 100vh;
		width: 100vw;
		margin: 0;
		padding: 0;
	}
	.splash {
		height: 100%;
		width: 100%;
		background-color: var(--cor-tema-fundo-1);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-around;
	}
	h1 {
		width: 100%;
		text-align: center;
		font-size: 100px;
		color: var(--cor-tema-fraca);
		margin: 0;
	}
	@keyframes rotating {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	canvas {
		animation: rotating 0.8s linear infinite;
	}
</style>
