<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri'
	onMount(() => {
		invoke("checar_tipo_de_janela").then(resultado=>{
			if(resultado == "instalacao")
				window.location.replace("instalacao");
			else
				window.location.replace("form");
			
		})
	});
</script>

