<script>
	import { ButtonGroup, Button } from 'agnostic-svelte-ts';
</script>

<div class="receipt-buttons">
	<a class="first" href="/receipt/create">Crear un recibo</a>
	<a class="second" href="/receipt/all-receipts">Todos los recibos</a>
	<a class="third" href="/receipt/search-receipts">Buscar un recibo</a>
</div>

<style lang="scss">
	.receipt-buttons {
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
</style>
