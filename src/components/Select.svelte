<script>
	import Arrow from './icons/Arrow.svelte';

	/**@type {{value: string | number, label: string}[]}*/
	export let options = [];
	/**@type {(string | number)}*/
	export let value = '';
	export let label = '';
	export let id = '';
	export let name = '';
	export let index = 0;
	$: {
		value = options[index].value;
	}
</script>

<div class="select">
	<label for={id}>{label}</label>
	<select {id} {name} bind:value={index}>
		{#each options as option, i (option.value)}
			<option value={i}>{option.label}</option>
		{/each}
	</select>
	<Arrow />
</div>

<style lang="scss">
	.select {
		position: relative;
		display: flex;
		flex-direction: column;
		label {
			font-family: 'mona-sans';
			color: #333;
		}
		select {
			appearance: none;
			border: 0;
			outline: 0;
			font: inherit;
			width: auto;
			padding: 1rem 4rem 1rem 1rem;
			color: black;
			border-radius: 0.25em;
			border: 1px solid #e9e9e9;
			transition: all 0.25s ease-out;
			cursor: pointer;
			&::-ms-expand {
				display: none;
			}
			&:focus {
				outline: none;
				box-shadow: 0 0 1em 0 #c1d9e9;
			}
			option {
				color: black;
				background-color: var(--option-bg);
			}
		}
		:global(.arrow-icon) {
			position: absolute;
			pointer-events: none;
			top: 50%;
			right: 12px;
		}
	}
</style>
