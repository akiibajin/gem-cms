<script>
	import HelpHref from '../../components/HelpHref.svelte';
</script>

<div class="home-settings">
	<h1>Configuración</h1>
	<details open>
		<summary>Ajustes Generales</summary>
		<div class="content">
			<HelpHref href="/settings/company" text="Datos de la compañia"
				>Podras configurar los datos basicos de la compañia</HelpHref
			>
			<HelpHref href="/settings/clients" text="Datos de los clientes"
				>Aca podras actualizar o añadir clientes que tengas, para crearlos tambien lo puedes hacer
				al crear un recibo</HelpHref
			>
		</div>
	</details>
</div>

<style lang="scss">
	.home-settings {
		max-width: 80vw;
		padding: 24px;
		margin: 20px auto;
		border: 1px solid black;
		height: 85dvh;
		max-height: 85dvh;
		details {
			width: 25%;
			box-sizing: border-box;
			transition: all 0.3s ease-out;
			font-size: 14px;
			color: #212121;
			background: #f2f2f2;
			border-radius: 10px;
			overflow: hidden;
			&[open] {
				padding-bottom: 20px;
				summary {
					margin-bottom: 10px;
					&::before {
						transform: rotate(-135deg) translate(-30%, 0%);
					}
				}
			}
			&:not(:first-child) {
				margin-top: 10px;
			}
			summary {
				display: flex;
				align-items: center;
				position: relative;
				padding: 10px 16px;
				transition: margin 0.3s ease-out;
				cursor: pointer;
				font-size: 16px;
				font-weight: 500;
				&::-webkit-details-marker {
					display: none;
				}
				&::before {
					content: '';
					display: block;
					height: 6px;
					flex: 0 0 6px;
					margin-right: 20px;
					border-right: 2px solid #212121;
					border-bottom: 2px solid #212121;
					transition: all 0.3s;
					transform: rotate(45deg) translate(0, -30%);
				}
			}
			.content {
				padding: 0 20px;
			}
		}
	}
</style>
