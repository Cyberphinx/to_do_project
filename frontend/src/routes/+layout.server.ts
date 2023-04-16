export function load({ locals, cookies }) {
	return {
		token: cookies.get('jwt')
	};
}