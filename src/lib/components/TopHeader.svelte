<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	export let inicial = 0;
	
	let tabsEl: HTMLElement;
	function cliqueTab(evento: Event) {
		window.location.replace("form");
		console.log('clique');
		
		if (evento.target) {
			let i = 0;
			for (let child of tabsEl.children) {
				if (child == evento.currentTarget) {
					(child as HTMLElement).style.setProperty('--cor-principal', 'var(--cor-tema-forte)');
					(child as HTMLElement).style.setProperty('--cor-fundo', 'white');
					(child as HTMLElement).style.setProperty('cursor', 'default');
					(child as HTMLElement).style.borderRadius = '10px 10px 0 0';
					console.log(i);
				} else if (!child.classList.contains('spacer')) {
					(child as HTMLElement).style.setProperty('--cor-principal', 'var(--cor-tema-fraca)');
					(child as HTMLElement).style.setProperty('--cor-fundo', 'var(--cor-tema-fundo-1)');
					(child as HTMLElement).style.setProperty('cursor', 'pointer');
					(child as HTMLElement).style.borderRadius = '10px';
				}
				i++;
			}
		}
	}
	
	onMount(()=>{
		let child = tabsEl.children[inicial+1];
		if(child){
			(child as HTMLElement).style.setProperty('--cor-principal', 'var(--cor-tema-forte)');
			(child as HTMLElement).style.setProperty('--cor-fundo', 'white');
			(child as HTMLElement).style.setProperty('cursor', 'default');
			(child as HTMLElement).style.borderRadius = '10px 10px 0 0';
		}
	})
</script>

<nav>
	<div id="tabs" bind:this={tabsEl}>
		<div class="logo-holder"></div>
		<button on:click={()=>{window.location.replace("form")}}><span>Formulário</span></button>
		<button on:click={()=>{window.location.replace("viz")}}><span>Visualizar</span></button>
		<button on:click={()=>{window.location.replace("ferramentas")}}><span>Ferramentas</span></button>
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div class="spacer" on:mousedown={appWindow.startDragging}></div>
	</div>
	<div id="window-buttons">
		<button id="minimize" on:click={appWindow.minimize}></button>
		<button id="expand" on:click={appWindow.toggleMaximize}></button>
		<button id="close" on:click={appWindow.close}></button>
	</div>
</nav>

<style>
	* {
		display: flex;
	}
	nav {
		height: 30px;
		width: 100vw;
		margin: 0;
		padding: 0;
		background-color: var(--cor-tema-fundo-1);
		justify-content: flex-end;
	}
	#tabs {
		flex-grow: 1;
		align-items: flex-end;
		position: relative;
		min-width: 0;
		overflow: hidden;
	}
	#tabs::before {
		position: absolute;
		content: '';
		bottom: 0;
		left: 0;
		width: 100%;
		height: 30%;
		background-color: white;
	}
	#tabs > * {
		background-color: var(--cor-fundo);
		border: none;
		border-radius: 10px;
		/* margin-left: 10px; */
		padding: 4px;
		align-items: center;
		position: relative;
		height: 28px;
		--cor-fundo: var(--cor-tema-fundo-1);
		--cor-principal: var(--cor-tema-fraca);
	}
	#tabs button {
		cursor: pointer;
		width: 100px;
	}
	#tabs > * span {
		border-radius: var(--tema-border-radius);
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
		background-color: var(--cor-principal);
	}
	/* Se é firefox, gambiarra. -moz não funcionou */
	@supports (not (text-wrap: nowrap)) {
		#tabs > * span {
			font-size: 11px;
	    }
	}

	#tabs .logo-holder {
		width: 10px;
		border-bottom-left-radius: 0;

	}
	#tabs .spacer {
		flex-grow: 1;
		flex-shrink: 1;
		border-bottom-right-radius: 0;
		min-width: 0;
	}

	#window-buttons {
		position: sticky;
		left: 0;
	}
	#window-buttons button {
		width: 30px;
		height: 30px;
		background-color: transparent;
		border: none;
		border-radius: 0px;
		align-items: center;
		justify-content: center;
		transition: background-color 0.2s;
	}
	#window-buttons button:hover {
		background-color: rgba(255, 255, 255, 0.3);
	}
	#expand::before {
		border: 1px solid rgb(180, 180, 180);
		border-radius: 1px;
		height: 8px;
		width: 8px;
		content: '';
	}
	#minimize::before,
	#close::before,
	#close:after {
		border: 1px solid rgb(180, 180, 180);
		border-top-width: 0;
		border-radius: 1px;
		height: 0px;
		width: 8px;
		content: '';
		position: absolute;
	}
	#close::before,
	#close:after {
		border-color: white;
		width: 13px;
		border-top-width: 0px;
	}
	#close::before {
		transform: rotate(45deg);
	}
	#close::after {
		transform: rotate(-45deg);
	}
	#window-buttons #close:hover {
		background-color: red;
	}
</style>
