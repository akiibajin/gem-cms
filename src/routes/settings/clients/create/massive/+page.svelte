<script>
	import Plus from '../../../../../components/icons/Plus.svelte';
	import readXlsxFile from 'read-excel-file';
	import { invoke } from '@tauri-apps/api/tauri';
	let loading = false;
	let success = false;
	const getDisCount = (num) => num * 100;
	const handleFile = async (e) => {
		try {
			loading = true;
			const rows = await readXlsxFile(e.target.files[0]);
			/**@type {import('$lib/types').IClient[]}*/
			const clientsOnExcel = rows.slice(1, rows.length).map((row) => {
				const [name, doc, address, landline, discount, email, mobile] = row;
				return {
					name: name || 'NA',
					doc: doc || 'NA',
					address: address || 'NA',
					landline: landline || 'NA',
					discount: !!discount ? getDisCount(discount) : 0,
					email: email || 'NA',
					mobile: mobile || 'NA'
				};
			});
			const clientsForm = JSON.stringify(clientsOnExcel);
			const response = await invoke('post_clients', { client: clientsForm });
			if (!!response) {
				success = true;
			}
		} catch (e) {
			console.log(e);
		} finally {
			loading = false;
		}
		console.log(e.target.files);
	};
</script>

<div class="massive-container">
	<h1>Esta es la carga masiva</h1>
	<div class="file-container">
		<h2>Arrastra el archivo excel</h2>
		<Plus size={200} />
		<input
			type="file"
			accept="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
			on:input={handleFile}
		/>
	</div>
</div>

<style lang="scss">
	.massive-container {
		max-width: 80vw;
		padding: 24px;
		margin: 0 auto;
		h1 {
			text-align: center;
		}
		.file-container {
			position: relative;
			border-radius: 16px;
			display: grid;
			gap: 12px;
			height: 60dvh;
			place-content: center;
			background-color: gray;
			&:hover {
				background: lighten(gray, 35%);
			}
			:global(svg) {
				justify-self: center;
				background: white;
				border-radius: 100px;
			}
			input {
				position: absolute;
				z-index: 1;
				top: 0;
				left: 0;
				width: 100%;
				height: 100%;
				opacity: 0;
			}
		}
	}
</style>
