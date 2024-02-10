<!-- 
	Esse modulo foi feito para resolver o problema de ter overflow scroll em 
	um dos sentidos e visible no outro. Para usar, os elementos colocados 
	dentro devem explicitar em seu estilo se forem pointer-events:all. Dá 
	para configurar quais lados do overflow, usando o atributo "overflow".
	
	O atributo overflow deve receber dois valores separados por espaço, 
	"visible" ou "scroll".
 -->
<script lang="ts">
	import { onMount } from 'svelte';

	export let overflow = 'visible scroll';
	
	let isScroll = [false, false];
	let content: HTMLElement;
	let fakeContent: HTMLElement;
	let view:HTMLElement;

	function onScroll(e: UIEvent) {
		if (e.target) {
			let el = e.target as HTMLElement;
			content.style.top = -el.scrollTop + 'px';
			content.style.left = -el.scrollLeft + 'px';
		}
	}
	onMount(() => {
		let ov = overflow.split(' ');
		isScroll[0] = ov[0][0] === 's';
		isScroll[1] = ov[1][0] === 's';

		view.style.overflow = (isScroll[0]?'clip':'visible')+' '+(isScroll[1]?'clip':'visible');

		if(isScroll[0] || isScroll[1]){
			new ResizeObserver((entries) => {
				// this will get called whenever div dimension changes
				entries.forEach((entry) => {
					if (isScroll[0]) fakeContent.style.width = entry.contentRect.width + 'px';
					else fakeContent.style.width = '100%';
	
					if (isScroll[1]) fakeContent.style.height = entry.contentRect.height + 'px';
					else fakeContent.style.height = '100%';
				});
			}).observe(content);
		}

	});
</script>

<main>
	<div id="scroller" on:scroll={onScroll}>
		<div id="fake-content" bind:this={fakeContent}></div>
	</div>
	<div id="view" bind:this={view}>
		<div id="content" bind:this={content}>
			<slot />
		</div>
	</div>
</main>

<style>
	main {
		height: 100%;
		width: 100%;
		position: relative;
	}
	#view {
		overflow: visible;
		height: 100%;
		width: 100%;
		position: relative;
		pointer-events: none;
	}
	#content {
		min-width: 100%;
		min-height: 100%;
		width: fit-content;
		height: fit-content;
		/* isso é para resolver uma margem que move o content inteiro. */
		padding: 0.1px;
		position: absolute;
		top: 0;
		left: 0;
	}
	#content > *{
		pointer-events: all;
	}
	#scroller {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		overflow: auto;
	}
</style>
