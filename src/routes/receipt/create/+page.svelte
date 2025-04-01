<script>
	import { Input, Button, Dialog, Tooltip } from 'agnostic-svelte-ts';
	import ItemsFormInputs from '../../../components/ItemsFormInputs.svelte';
	import { initialValue, client } from '../../../utils/constants';
	import { getStringDate } from '../../../utils/helpers/date';
	import { invoke } from '@tauri-apps/api/tauri';
	import CompanyForm from '../../../components/CompanyForm.svelte';
	import Info from '../../../components/icons/Info.svelte';

	let clientData = { ...client };
	let itemsData = [{ ...initialValue }];
	const currentDate = new Date();
	let limitDate = getStringDate(currentDate);
	const deliverDate = new Date(new Date(currentDate).setDate(currentDate.getDate() + 1));
	let deliverValue = getStringDate(deliverDate);
	let observations = '';
	let dialogInstance;
	let index;
	let jewelsFieldSet;

	const addItemForm = () => {
		itemsData = [...itemsData, { ...initialValue, id: itemsData.length }];
		setTimeout(() => {
			jewelsFieldSet.scroll({ top: jewelsFieldSet.scrollHeight, behavior: 'smooth' });
		}, 100);
	};
	const assignDialogInstance = (ev) => {
		dialogInstance = ev.detail.instance;
	};
	/** @param {number} itemId */
	const deleteItemForm = (itemId) => {
		itemsData = itemsData.filter(({ id }) => itemId !== id).map((item, i) => ({ ...item, id: i }));
		index = undefined;
		dialogInstance.hide();
	};
	const openDialog = (id) => {
		if (dialogInstance) {
			dialogInstance.show();
			index = id;
		}
	};
	const handleSubmit = async (e) => {
		try {
			const receiptData = JSON.stringify([
				{
					client_doc: clientData.doc,
					entry_date: limitDate,
					delivery_date: deliverValue,
					observations
				}
			]);
			const itemsToSend = itemsData.map((item) => ({
				...item,
				id: undefined,
				weight: Number(item.weight),
				dim_unit: item.dimUnit,
				weight_unit: item.weightUnit,
				width: Number(item.width),
				height: Number(item.height),
				length: Number(item.length),
				value: Number(item.value)
			}));
			const response = await invoke('post_receipts', { receipt: receiptData, items: itemsToSend });
		} catch (e) {
			console.error(e);
		}
	};
</script>

<div class="create-receipt__content-form">
	<Dialog
		dialogRoot="#dialog-root"
		titleId="close-title"
		role="dialog"
		title={`Desea eliminar la muestra ${index}?`}
		isAnimationFadeIn
		isAnimationSlideUp
		on:instance={assignDialogInstance}
		closeButtonPosition="last"
		closeButtonLabel="eliminar item"
		id="confirm-delete-item"
		><Button on:click={() => deleteItemForm(index)} mode="secondary">Eliminar</Button></Dialog
	>
	<h1>Llena los datos de la factura que crearas</h1>
	<form on:submit|preventDefault={handleSubmit}>
		<section class="company__section">
			<h2>Datos del cliente</h2>
			<fieldset class="company-info">
				<CompanyForm bind:clientData />
			</fieldset>
		</section>
		<section class="jewels__section">
			<h2>Muestras</h2>
			<fieldset bind:this={jewelsFieldSet} class="jewels">
				{#each itemsData as itemData (itemData.id)}
					<p>Muestra #{itemData.id}</p>
					{#if itemsData.length > 1}<Button
							type="button"
							css="erase-btn"
							on:click={() => openDialog(itemData.id)}
							mode="secondary">X</Button
						>{/if}
					<ItemsFormInputs
						bind:jewel={itemData.jewel}
						bind:weight={itemData.weight}
						bind:weightUnit={itemData.weightUnit}
						bind:length={itemData.length}
						bind:width={itemData.width}
						bind:height={itemData.height}
						bind:dimUnit={itemData.dimUnit}
						bind:color={itemData.color}
						bind:cut={itemData.cut}
						bind:language={itemData.language}
						bind:service={itemData.service}
						bind:value={itemData.value}
					/>
				{/each}
				<Button
					type="button"
					isDisabled={itemsData.length > 10}
					mode="primary"
					on:click={addItemForm}
					isRounded>Añadir muestra</Button
				>
			</fieldset>
		</section>
		<section class="additiona-data__section">
			<h2>Información adicional</h2>
			<fieldset class="additional-data">
				<div class="calendar-input-content">
					<Tooltip placement="top-start">
						<Info />
						<div slot="content">
							Si no colocas fecha, será la de mañana
							<br />
							(dia / mes / año)
						</div>
					</Tooltip>
					<Input
						min={limitDate}
						type="datetime-local"
						bind:value={deliverValue}
						label="Fecha de entrega"
					/>
				</div>
				<Input type="textarea" bind:value={observations} label="Observaciones" />
			</fieldset>
		</section>
		<Button css="send-form" type="submit" mode="primary">Enviar e Imprimir</Button>
	</form>
</div>

<style lang="scss">
	.create-receipt {
		&__content-form {
			padding: 40px;
			margin: auto;
			h1 {
				text-align: center;
			}
			form {
				@media screen and (min-width: 1200px) {
					display: grid;
					grid-template-columns: 1fr 1fr;
					grid-template-rows: 30vh 15vh auto;
					column-gap: 48px;
				}
				.company-info {
					padding: 16px;
					margin-bottom: 24px;
				}
				.jewels__section {
					@media screen and (min-width: 1200px) {
						grid-row-start: 1;
						grid-row-end: -1;
						height: auto;
						grid-column: 2;
						max-height: 70vh;
					}
				}
				.jewels {
					position: relative;
					overflow-y: auto;
					overflow-x: hidden;
					max-height: 40vh;
					@media screen and (min-width: 1200px) {
						height: auto;
						max-height: 100%;
					}
					p {
						margin: 8px 12px 0;
					}
					:global(.btn) {
						position: relative;
						right: calc(-100% + 200px);
						bottom: 8px;
					}
					:global(.erase-btn) {
						position: relative;
						right: calc(-100% + 64px);
						bottom: auto;
						top: 0;
						z-index: 1;
					}
				}
				.additional-data {
					grid-row-start: 2;
					grid-row-end: 3;
					position: relative;
					padding: 24px;
					display: grid;
					gap: 12px;
					grid-template-columns: repeat(4, 1fr);
					place-content: center;
					.calendar-input-content {
						display: flex;
						align-items: center;
						column-gap: 5px;
					}
				}
				:global(.send-form) {
					position: fixed;
					bottom: 24px;
					left: 24px;
				}
			}
		}
	}
</style>
