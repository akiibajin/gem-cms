<script>
	import {invoke} from "@tauri-apps/api/tauri";
    import {onMount} from "svelte";
    import {Loader} from "agnostic-svelte-ts";
    import TableReceipts from "../../../components/TableReceipts.svelte";
    let loading = false;
    let receipts = [];
    onMount(()=>{
        const getReceipts = async()=>{
            try{
                loading=true;
                receipts = await invoke("get_receipts");
            }
            catch(e){
                console.error(e)
            }
            finally {
                loading=false
            }
        }
        getReceipts();
    })
    $: {
        console.log(receipts)
    }
</script>

<div class="all-receipts">
    <h1>Todos los recibos</h1>
    {#if loading} <Loader></Loader> {/if}
    <TableReceipts receipts={receipts}/>
</div>

<style lang="scss">
    .all-receipts {
      max-width: 80vw;
      padding: 24px;
      margin: auto;
      h1 {
        text-align: center;
      }
    }
</style>
