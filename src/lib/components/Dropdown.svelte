<script lang="ts">
	import { onMount } from 'svelte';
	import DropdownItem from './DropdownItem.svelte';

	export let dados: any[] = [''] ;
	//  = [['A', [['1','N'], '2', '3']], ['B'], ['C', ['1', '2', ['3D', ['1', '2']]]]];
	export let directionDown = true;
	export let value:string[] = ['oi'];

	export function reset() {
		let btn = mainEl.querySelector('.adb')
		if(btn) (btn as HTMLButtonElement).click();
		// let s: string[] = [];
		// let v: any = dados;
		// let limiter = 0
		// while (v != undefined && limiter<10) {
		// 	limiter++;
		// 	if(isString(v)){
		// 		s.push(v);
		// 		v = undefined;
		// 	}
		// 	else if(isString(v[0])){
		// 		s.push(v[0]);
		// 		if(!isString(v[1]))
		// 			v = v[1];
		// 		else
		// 		v = undefined;
		// 	}
		// 	else{
		// 		v = v[0];
		// 	}
		// 	console.log(v);
	
		// }
		// console.log(s);

		// value = s;
	}
	
	export function setDados(v:any){
		dados = v;
		window.requestAnimationFrame(()=>{
			reset();
		})
	}
	
	export function obterValor() {
		return value;
	}

	export let visualTreatment = (v:string[])=>{
		return v.join(' ');
	}


	let mainEl:HTMLElement;
	let ulEl:HTMLElement;

	function onBubbleUp(v: string[]) {
		value = v;
		ulEl.style.display = "none";
		
		setTimeout(()=>{
			ulEl.style.display = "block";
			
			// mainEl.focus();
			// mainEl.blur();

		}, 10);
	}
	onMount(async () => {
		reset();
	});
</script>

<main bind:this={mainEl} class="dropdown-main">
	<button>{visualTreatment(value)}</button>
	<ul bind:this={ulEl} class={directionDown ? 'drop-down' : 'drop-sideways}'}>
		{#each dados as dado}
			<DropdownItem bubbleUp={onBubbleUp} dados={dado} />
		{/each}
	</ul>
</main>

<style src="./Dropdown.css"></style>
