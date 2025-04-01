<script>
</script>

<div class="client-settings-container">
	<h1>Ajustes de clientes</h1>
	<p>Selecciona una de las opciones</p>
	<div class="anchors-group">
		<a class="first" href="/settings/clients/create">Crear un nuevo cliente</a>
		<a class="second" href="/settings/clients/all-clients">Mirar todos los clientes</a>
		<a class="third" href="/settings/clients/set-client">Actualizar un cliente</a>
	</div>
</div>

<style lang="scss">
	.client-settings-container {
		max-width: 80vw;
		margin: 0 auto;
		padding: 24px;
		h1 {
			text-align: center;
		}
		.anchors-group {
			width: 100vw;
			height: calc(70dvh);
			position: absolute;
			top: 50%;
			left: 50%;
			display: flex;
			gap: 24px;
			justify-content: center;
			align-items: center;
			transform: translate(-50%, -50%);
			a {
				height: 200px;
				display: flex;
				justify-content: center;
				align-items: center;
				font-weight: 600;
				list-style: none;
				font-size: larger;
				list-style: none;
				transition: all 0.25s ease-in-out;
				border-radius: 8px;
				padding: 16px;
				&.first {
					color: #88498f;
					box-shadow: #88498f 0px 0px 0px 2px, #88498f 0px 4px 6px -1px, #88498f 0px 1px 0px inset;
					&:hover {
						background-color: #88498f;
					}
				}
				&.second {
					color: #779fa1;
					box-shadow: #779fa1 0px 0px 0px 2px, #779fa1 0px 4px 6px -1px, #779fa1 0px 1px 0px inset;
					&:hover {
						background-color: #779fa1;
					}
				}
				&.third {
					color: #ff6542;
					box-shadow: #ff6542 0px 0px 0px 2px, #ff6542 0px 4px 6px -1px, #ff6542 0px 1px 0px inset;
					&:hover {
						background-color: #ff6542;
					}
				}
				&:hover {
					list-style: none;
					color: white;
				}
			}
		}
	}
</style>
