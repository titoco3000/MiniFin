<script lang="ts">
	import { onMount } from 'svelte';
	import NewDropdownItem from './NewDropdownItem.svelte';
    
    let listEl:HTMLElement;
    let buttonEl:HTMLElement;
    export let value: string[] = [''];

    export let dados: any = [
		[
            'main',
            [
                ['title2', [['item 1',[]], ['item 2',[]]]],
                ['item A', []]
            ]
        ]
	];

    export function reset(){
        let r:string[] = [];
        let d = dados;
        //traverse through tree
        while( true ){
            //if first el is a string
            if (typeof d[0] === 'string'){
                r.push(d[0]); 
                if(d[1].length == 0){
                    value = r;
                    return;
                }
                else
                    d = d[1];
            }
            else
                d = d[0];
        }
    }
    export function setDados(v:any){
        value = v;
    }
    export let visualTreatment = (v:string[])=>{
		return v.join(' ');
	}
    export let onEdit = (v:string[])=>{

    };
    
    function onBubbleUp(v: string[]) {
		value = v;
        window.setTimeout(() => {
            buttonEl.focus()
            buttonEl.blur();
        }, 0);
        onEdit(v);
	}

    onMount(()=>{
        reset();
    })
</script>

<main class="ndp">
	<button bind:this={buttonEl}>
        {visualTreatment(value)}
    </button>
		<ul bind:this={listEl}>
            {#each dados as dado}
                 <li>
                     <NewDropdownItem dados={dado} onBubbleUp={onBubbleUp}/>
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
    main{
        width: 100%;
        width: 200px; /*temp*/
        position: relative;
        display: flex;
    }
    button{
        border: 1px solid black;
        width: 100%;
        height: 100%;
    }
    main:focus-within button{
        border-bottom-width: 0;
    }
    ul{
        list-style: none;
        position: absolute;
        top: 100%;
        width: 100%;
    }
    :global(.ndp){
        --transition-time: 0.4s;
    }
    /* Seleciona botoes da primeira coluna */
    :global(.ndp main button){
        max-height: 0;
        border-top-width: 0;
        border-bottom-width: 0;
        overflow: hidden;
        transition: max-height var(--transition-time), border-top-width var(--transition-time), border-bottom-width var(--transition-time);
    }
    :global(.ndp:focus-within main button){
        max-height: 50px;
        border-top-width: 1px;
        border-bottom-width: 1px;
        transition: max-height var(--transition-time), border-top-width var(--transition-time), border-bottom-width var(--transition-time);
    }
    /* Seleciona botoes das colunas subsequentes */
    :global(.ndp ul ul li ){
        max-width: 0;
        transition: max-width var(--transition-time);
    }
    :global(.ndp ul main:focus-within > ul > li){
        max-width: 1000px;
        transition: max-width var(--transition-time);
    }
    
    :global(.ndp ul ul li > main > button){
        border-left-width: 0px;
        border-right-width: 0px;
        transition: border-left-width var(--transition-time), border-right-width var(--transition-time);
    }
    :global(.ndp ul main:focus-within > ul > li > main > button){
        border-left-width: 1px;
        border-right-width: 1px;    
    }

    /* Seleciona todos botoes*/
    :global(.ndp ul > li:not(:last-child) > main > button){
        border-bottom-width: 0;
    }

</style>
