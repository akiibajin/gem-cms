export interface ITest {
	id: number;
	created_at: string;
	name: string;
}
export interface IItem {
	id: number;
	jewel: string;
	weight: number;
	weightUnit: string;
	length: string;
	width: number;
	height: number;
	dimUnit: string;
	color: string;
	cut: string;
	language: string;
	service: string;
	value: string;
}

export interface IClient {
	doc: string;
	name: string;
	address: string;
	mobile: string;
	email: string;
	discount: number;
	landline: string;
}
