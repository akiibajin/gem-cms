<script>
    import {Tooltip} from "agnostic-svelte-ts";
    import ClipBoard from "./icons/ClipBoard.svelte";

    export let receipts =[];
    const copyToClipBoard = (text) => {
        navigator.clipboard.writeText(text);
    }
</script>

<table>
    <thead>
        <tr class="table-headers">
            <th>Id Recibo</th>
            <th># Cliente</th>
            <th>Fecha de Creación</th>
            <th>Fecha de entrega</th>
            <th>Observaciones</th>
        </tr>
    </thead>
    <tbody>
    {#each receipts as receipt (receipt.id)}
        <tr>
            <td>{receipt.id}</td>
            <td class="mobile-header"># Cliente</td><th>{receipt.client_doc}</th>
            <td class="mobile-header">Fecha de Creación</td><th>{receipt.entry_date}</th>
            <td class="mobile-header">Fecha de entrega</td><th>{receipt.delivery_date}</th>
            <td class="mobile-header">Observaciones</td>
            <th>
                <Tooltip placement="top">
                    {receipt.observations}
                    <div slot="content">{receipt.observations}</div>
                </Tooltip>
                <button on:click={()=>copyToClipBoard(receipt.observations)}><ClipBoard size={20}/></button>
            </th>
        </tr>
        {/each}
    </tbody>
</table>

<style lang="scss">
 table {
   border-radius: 8px;
   width: 100%;
   > * {
     border-radius: 8px;
   }
   thead {
     border-bottom: 1px solid black;
     color: white;
     background: #88498f;
     tr {
       height: 50px;
       th {
         text-align: center;
       }
     }

   }
   tbody {
     .mobile-header {
       display:none;
     }
     tr {
       height: 50px;
       &:nth-child(even) {
         background: lighten(#88498f, 35%);
       }
       th, td {
         text-align: center;
         padding: 0 24px;
         min-width: 150px;
         max-width: 200px;
         &:nth-last-child(1) {
           position: relative;
           button {
             position: absolute;
             right: 5px;
             top: 50%;
             transform: translateY(-50%);
             border: 0;
             background: transparent;
             padding: 4px;
             display: grid;
             place-content: center;
             border-radius: 100px;
             &:hover {
               background:  lighten(#88498f, 25%);
             }
           }
           :global(>div:nth-child(1)) {
             max-width: 100%;
             overflow: hidden;
             text-align: center;
             text-wrap: nowrap;
             text-overflow: ellipsis;
           }
         }
       }
     }
   }
 }

</style>