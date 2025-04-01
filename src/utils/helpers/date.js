/**
 *
 * @param {Date} date
 */
export const getStringDate = (date) => {
	return date.toISOString().split('.')[0];
};
/**
 *
 * @param {string} num
 */
export const getNumberOfDate = (num) => {
	if (Number(num) < 10) return '0' + num;
	return num;
};
