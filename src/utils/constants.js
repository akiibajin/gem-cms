export const routes = [
	'/',
	'/receipt',
	'/receipt/create',
	'/receipt/all-receipts',
	'/settings',
	'/settings/clients',
	'/settings/clients/create',
	'/settings/clients/create/massive'
];
export const breadCrumbRoutes = {
	'/': [{ url: '/', label: 'Inicio' }],
	'/receipt': [
		{ url: '/', label: 'Inicio' },
		{ url: '/receipt', label: 'Recibos' }
	],
	'/receipt/create': [
		{ url: '/', label: 'Inicio' },
		{ url: '/receipt', label: 'Recibos' },
		{ url: '/receipt/create', label: 'Crear recibos' }
	],
	'/receipt/all-receipts': [
		{ url: '/', label: 'Inicio' },
		{ url: '/receipt', label: 'Recibos' },
		{ url: '/receipt/all-receipts', label: 'Todos los recibos' }
	],
	'/settings': [
		{ url: '/', label: 'Inicio' },
		{ url: '/settings', label: 'Ajustes' }
	],
	'/settings/clients': [
		{ url: '/', label: 'Inicio' },
		{ url: '/settings', label: 'Ajustes' },
		{ url: '/settings/clients', label: 'Clientes' }
	],
	'/settings/clients/create': [
		{ url: '/', label: 'Inicio' },
		{ url: '/settings', label: 'Ajustes' },
		{ url: '/settings/clients', label: 'Clientes' },
		{ url: '/settings/clients/create', label: 'Crear clientes' }
	],
	'/settings/clients/create/massive': [
		{ url: '/', label: 'Inicio' },
		{ url: '/settings', label: 'Ajustes' },
		{ url: '/settings/clients', label: 'Clientes' },
		{ url: '/settings/clients/create', label: 'Crear clientes' },
		{ url: '/settings/clients/create/massive', label: 'Carga masiva' }
	]
};

/**@type{import('$lib/types').IItem}*/
export const initialValue = {
	id: 0,
	jewel: 'x',
	weight: 0,
	weightUnit: '',
	length: '',
	width: 0,
	height: 0,
	dimUnit: '',
	color: '',
	cut: '',
	language: '',
	service: '',
	value: ''
};
/**@type {Partial<import('$lib/types').IClient>} */
export const client = {
	doc: undefined,
	name: '',
	address: '',
	mobile: '',
	email: '',
	discount: 0,
	landline: ''
};
