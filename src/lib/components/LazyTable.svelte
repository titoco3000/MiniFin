<script lang="ts">
    let titulos = ['a','b','c','d', 'e']
    let tableHeaderEl:HTMLTableRowElement;
    
    function getRowAtId(id:number){
        return [id,id,id,id,id];
    }

    function totalLinhas():number{
        return 1000;
    }

    function carregarValores(){

    }

</script>
<main>
    <table>
        <tr bind:this={tableHeaderEl}>
            {#each titulos as titulo}
                <th>
                    {titulo}
                </th>
            {/each}
            </tr>
    </table>
</main>
<style>
main{
    width: 100%;
    height: 100%;
}
</style>