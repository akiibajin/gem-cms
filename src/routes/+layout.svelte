<script>
	import 'agnostic-svelte-ts/css/common.min.css';
	import './styles.scss';
	import Header from './Header.svelte';
	import { page } from '$app/stores';
	import { Breadcrumb, Icon } from 'agnostic-svelte-ts';
	import { breadCrumbRoutes, routes } from '../utils/constants';
	import Setting from '../components/icons/Setting.svelte';
	let breadCrumb = breadCrumbRoutes['/'];
	let hasSettings = false;
	$: {
		hasSettings = !$page.url.pathname.includes('settings');
		const hasAnyRoute = routes.includes($page.url.pathname);
		console.log({ hasAnyRoute, routes, url: $page.url.pathname });
		if (hasAnyRoute) {
			// @ts-ignore
			breadCrumb = breadCrumbRoutes[$page.url.pathname];
		} else {
			breadCrumb = breadCrumbRoutes['/'];
		}
	}
</script>

<div class="app">
	<Header>
		<ul class="navigation-list">
			<li><a href="/tests">Pruebas</a></li>
			<li><a href="/receipt">Recibos</a></li>
		</ul>
	</Header>
	<section>
		<Breadcrumb routes={breadCrumb} />
	</section>
	<main>
		<slot />
	</main>
	<footer>
		{#if hasSettings}<div class="settings-container">
				<a href="/settings">
					<Icon size={24}>
						<Setting />
					</Icon>
				</a>
				<p class="tooltip">Ir a ajustes</p>
			</div>
		{/if}
	</footer>
</div>

<style lang="scss">
	:global(.header) {
		width: 100%;
		.navigation-list {
			display: flex;
			flex-direction: row;
			justify-content: space-evenly;
			width: 100%;
		}
	}
	section {
		:global(.breadcrumb) {
			margin-top: 12px;
			margin-left: 5%;
		}
	}
	footer {
		position: sticky;
		bottom: 8px;
		right: 40px;
		display: flex;
		justify-content: flex-end;
		.settings-container {
			position: relative;
			margin-right: 50px;
			.tooltip {
				z-index: 1;
				position: absolute;
				display: none;
				top: -50px;
				width: max-content;
				left: 50%;
				transform: translateX(-50%);
				padding: 8px 16px;
				background-color: black;
				border-radius: 8px;
				color: white;
				&::after {
					content: '';
					position: absolute;
					display: block;
					width: 0px;
					left: 50%;
					bottom: -8px;
					border: 15px solid transparent;
					border-bottom: 0;
					border-top: 15px solid black;
					transform: translateX(-50%);
				}
			}
			a {
				&:hover + .tooltip {
					display: flex;
				}
			}
		}
	}
</style>
