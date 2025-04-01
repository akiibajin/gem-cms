<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button, Input, Loader } from 'agnostic-svelte-ts';
	import { client } from '../../../../utils/constants';
	/**@type {Partial<import("$lib/types").IClient>}*/
	let clientData = { ...client };
	let loading = false;
	let success = false;
	const handleSubmit = async () => {
		try {
			loading = true;
			const clientString = JSON.stringify([clientData]);
			console.log({ clientString });
			const response = await invoke('post_clients', { client: clientString });
			if (!!response) {
				clientData = { ...client };
				success = true;
			}
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	};
</script>

<div class="create-client">
	<h1>Create cliente</h1>
	<dialog open={success}>
		Se cargo satisfactoriamente
		<button
			on:click={() => {
				success = false;
			}}>Cerrar</button
		>
	</dialog>
	<form on:submit|preventDefault={handleSubmit}>
		<Input bind:value={clientData.name} label="nombre de la empresa" />
		<Input bind:value={clientData.doc} type="number" label="NIT" />
		<Input bind:value={clientData.address} label="Dirección" />
		<Input bind:value={clientData.mobile} label="numero movil" />
		<Input bind:value={clientData.landline} label="Numero Fijo" />
		<Input bind:value={clientData.email} type="email" label="Email" />
		<Input bind:value={clientData.discount} type="number" min={0} max={100} label="Descuento" />
		<Button on:click type="submit"
			>{#if loading} <Loader /> {:else} Crear cliente{/if}</Button
		>
	</form>
	<p><a href="/settings/clients/create/massive">Crear clientes de forma masiva</a></p>
</div>

<style lang="scss">
	.create-client {
		width: 80vw;
		padding: 24;
		margin: auto;
		h1 {
			text-align: center;
		}
		form {
			display: grid;
			place-content: center;
			grid-template-columns: 1fr 1fr;
			gap: 12px;
		}
		p {
			margin-top: 32px;
			a {
				color: white;
				background: #779fa1;
				border-radius: 8px;
				padding: 12px;
			}
		}
	}
</style>