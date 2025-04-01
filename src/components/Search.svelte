<script>
	/**@type {{[key:string]: any}[]} */
	export let options = [];
	/**@type {string}*/
	export let idKey = 'id';
	/**@type {string}*/
	export let showInfoKey = 'name';
	export let label = '';
	export let uniqueId = '';
	export let value;
	export let name = '';
	/**@type {import("svelte/elements").FormEventHandler<HTMLInputElement>}*/
	export let handleInput;
</script>

<label>
	{label}
	<input
		list={`${uniqueId}-options`}
		{...$$restProps}
		id={`${uniqueId}-search-input`}
		on:input={handleInput}
		bind:value
		{name}
	/>
</label>
<datalist id={`${uniqueId}-options`}>
	{#each options as option (option[idKey])}
		<option value={option[showInfoKey]} />
	{/each}
</datalist>

<style lang="scss">
	label {
		input {
			border-radius: 0.25em;
			border: 1px solid #e9e9e9;
			padding-left: 8px;
			width: 100%;
			height: 38px;
			&:focus {
				outline: none;
				box-shadow: 0 0 1em 0 #c1d9e9;
			}
		}
	}
</style>
