<script>
	import { Input } from 'agnostic-svelte-ts';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { delay } from '../utils/helpers/tests';
	import { client } from '../utils/constants';
	import Search from './Search.svelte';
	/**@type {import('$lib/types').IClient}*/
	export let clientData;
	/**@type {import('$lib/types').IClient[]} */
	let clients = [];
	let loading = false;
	let disabled = true;

	const selectClient = ({ target: { value } }, key) => {
		const clientFounded = clients.find((client) => String(client[key]) === String(value));
		if (!!clientFounded) {
			clientData = { ...clientFounded };
		} else if (!value) {
			clientData = { ...client };
		}
	};
	onMount(() => {
		const getClients = async () => {
			try {
				loading = true;
				clients = await invoke('get_clients');
			} catch (e) {
				console.log(e);
			} finally {
				loading = false;
			}
		};
		getClients();
	});
</script>

<section>
	<!-- <Input
		uniqueId="client-doc"
		isDisabled={loading || disabled}
		label="NIT"
		bind:value={clientData.doc}
	/> -->
	<Search
		label="NIT"
		disabled={loading}
		name="nit"
		bind:value={clientData.doc}
		handleInput={(e) => selectClient(e, 'doc')}
		options={clients}
		placeholder="Busca el cliente por el nit"
		idKey="doc"
		uniqueId="nit-client"
		showInfoKey="doc"
	/>
	<!-- <Input
		uniqueId="client-name"
		isDisabled={loading}
		label="Nombre de cliente"
		bind:value={clientData.name}
	/> -->
	<Search
		label="Nombre de cliente"
		disabled={loading}
		name="name"
		uniqueId="name-client"
		bind:value={clientData.name}
		handleInput={(e) => selectClient(e, 'name')}
		placeholder="busca el cliente por el nombre"
		options={clients}
		idKey="name"
		showInfoKey="name"
	/>
	<Input
		uniqueId="client-address"
		isDisabled={loading || disabled}
		label="Dirección"
		bind:value={clientData.address}
	/>
	<Input
		uniqueId="client-mobile"
		isDisabled={loading || disabled}
		label="Celular"
		bind:value={clientData.mobile}
	/>
	<Input
		type="client-email"
		isDisabled={loading || disabled}
		uniqueId="client-mobile"
		label="email"
		bind:value={clientData.email}
	/>
	<Input
		type="number"
		isDisabled={loading || disabled}
		uniqueId="client-discount"
		label="descuento"
		bind:value={clientData.discount}
	/>
	<Input
		uniqueId="client-landLine"
		isDisabled={loading || disabled}
		label="Numero de telefono"
		bind:value={clientData.landline}
	/>
</section>

<style lang="scss">
	section {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 16px;
	}
</style>
