<script>
	import { Tooltip } from 'agnostic-svelte-ts';
	import Info from './icons/Info.svelte';
	export let href = '/';
	export let text = '';
</script>

<Tooltip placement="right"
	><a {href}>{text}</a>
	<Info size={18} />
	<div slot="content">
		<slot />
	</div></Tooltip
>
