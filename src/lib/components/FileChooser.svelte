<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';

    export let pastas = false;
    export let placeholder = pastas?'Escolha uma pasta':'Escolha um arquivo';
    export let value = '';

    let nomeVisivel = placeholder;

    function abrirDialog(){
		// Open a file dialog
		open({
			multiple: false, // set to true for multiple file selection
			directory: pastas // set to true for directory selection
		}).then((selected) => {
			if (selected !== null) {
				// user selected a file
				if(typeof(selected)==='string'){
					value = selected;

					let fullpath = selected.split(/(\\|\/)/g);

                    nomeVisivel = fullpath.slice(Math.max(fullpath.length - 6, 0)).join('');
				}
			} else {
				// user cancelled the selection
				nomeVisivel = placeholder;
                value = '';
			}
		});
	}

</script>
<button type="button" on:click={abrirDialog}>
    <span>
        {nomeVisivel}
    </span>
</button>
<style>
    button{
		width: 200px;
		height: 200px;
		justify-content: flex-end;
		align-items: flex-end;
		display: inline-flex;
		/* text-align: right; */
        
	}
    span{
        width: 100%;
        direction:rtl;
        text-overflow: ellipsis;
        overflow: hidden;
        text-wrap: nowrap;
        text-align: right;
    }
</style>