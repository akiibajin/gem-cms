<script lang="js">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button, Card, Loader } from 'agnostic-svelte-ts';

	/**@type {import('$lib/types').ITest[]}*/
	let tests = [];
	let loading = false;
	const getTests = async () => {
		loading = true;
		tests = await invoke('get_tests');
		loading = false;
	};
</script>

<div class="tests">
	<Card isStacked isBorder>
		<Button css="test-btn" isRounded mode="primary" on:click|once={getTests}
			>{#if loading}<Loader size="xlarge" />{:else}obtener tests {/if}</Button
		>
		{#if !!tests.length}
			<h3>Estos son los test obtenidos de la db</h3>
			{#each tests as test (test.id)}
				<div><p>{test.name}</p></div>
			{/each}
		{/if}
	</Card>
</div>

<style lang="scss">
	.tests {
		display: flex;
		gap: 24px;
		flex-direction: column;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		:global(.test-btn) {
			width: 150px;
			margin: 24px;
		}
	}
</style>
