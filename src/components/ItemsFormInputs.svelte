<script>
	import { Input } from 'agnostic-svelte-ts';
	import Select from '../components/Select.svelte';
	export const id = 0;
	export let jewel = '';
	export let weight = 0;
	export let weightUnit = '';
	export let length = '';
	export let width = 0;
	export let height = 0;
	export let dimUnit = '';
	export let color = '';
	export let cut = '';
	export let language = '';
	export let service = '';
	export let value = '';
</script>

<section class="items-form-inputs">
	<Input uniqueId={`${id}-item-jewel`} label="Joya" bind:value={jewel} />
	<Input uniqueId={`${id}-item-weight`} label="Peso" type="number" bind:value={weight} />
	<Input uniqueId={`${id}-item-length`} label="Largo" bind:value={length} />
	<Input uniqueId={`${id}-item-width`} label="Ancho" bind:value={width} />
	<Input uniqueId={`${id}-item-height`} label="Altura" bind:value={height} />
	<Select
		id={`${id}-item-dimUnit`}
		label="Unidad dimensión"
		name="dimUnit"
		index={2}
		bind:value={dimUnit}
		options={[
			{ value: 'mt', label: 'mt' },
			{ value: 'cm', label: 'cm' },
			{ value: 'mm', label: 'mm' }
		]}
	/>
	<Select
		id={`${id}-item-weightUnit`}
		label="Unidad peso"
		name="weightUnit"
		index={0}
		bind:value={weightUnit}
		options={[
			{ value: 'ct', label: 'ct' },
			{ value: 'gr', label: 'gr' },
			{ value: 'kg', label: 'kg' }
		]}
	/>
	<Select
		id={`${id}-item-cut`}
		label="Talla"
		name="cut"
		index={5}
		bind:value={cut}
		options={[
			{ value: 'Briollet', label: 'Briollet' },
			{ value: 'Rough', label: 'Bruto' },
			{ value: 'Corazón', label: 'Heart' },
			{ value: 'Cushion', label: 'Cushion' },
			{ value: 'Sculpture', label: 'Escultura' },
			{ value: 'Emerald', label: 'Esmeralda' },
			{ value: 'Pear', label: 'Lágrima' },
			{ value: 'Marquise', label: 'Marquis' },
			{ value: 'Oval', label: 'Óvalo' },
			{ value: 'Rectangular', label: 'Rectangular' },
			{ value: 'Round', label: 'Redonda' }
		]}
	/>
	<Select
		id={`${id}-item-color`}
		label="Color"
		name="color"
		index={8}
		bind:value={color}
		options={[
			{ value: 'Yellow', label: 'Amarrillo' },
			{ value: 'Blue', label: 'Azul' },
			{ value: 'Colourless', label: 'Incoloro' },
			{ value: 'Orange', label: 'Naranja' },
			{ value: 'Black', label: 'Negro' },
			{ value: 'Purple', label: 'Púrpura' },
			{ value: 'Red', label: 'Rojo' },
			{ value: 'Pink', label: 'Rosa' },
			{ value: 'Green', label: 'Verde' },
			{ value: 'Violet', label: 'Violeta' }
		]}
	/>
	<Select
		index={0}
		label="Traducción"
		id={`${id}-item-language`}
		name="language"
		bind:value={language}
		options={[
			{ value: 'Español', label: 'Español' },
			{ value: 'Chino', label: 'Chino' },
			{ value: 'Francés', label: 'Francés' },
			{ value: 'Italiano', label: 'Italiano' }
		]}
	/>
	<Select
		index={0}
		label="Servicio"
		id={`${id}-item-service`}
		bind:value={service}
		name="service"
		options={[
			{ value: 'A', label: 'A' },
			{ value: 'B', label: 'B' },
			{ value: 'C', label: 'C' },
			{ value: 'AV', label: 'AV' },
			{ value: 'BV', label: 'BV' },
			{ value: 'CV', label: 'CV' },
			{ value: 'T', label: 'T' },
			{ value: 'Copy', label: 'Copia' }
		]}
	/>
	<Input bind:value uniqueId={`${id}-item-value`} label="Valor (COP)" />
</section>

<style lang="scss">
	.items-form-inputs {
		padding: 24px;
		display: grid;
		gap: 12px;
		grid-template-columns: repeat(4, 1fr);
		span {
			align-self: center;
			margin-top: 8px;
			justify-self: flex-end;
		}
	}
</style>
