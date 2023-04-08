import { error } from '@sveltejs/kit';

const base = 'http://127.0.0.1:6006';

async function send( {method, path, data, token} : {method: any, path: any, data: any, token: any}) {
	const opts: any = { method, headers: {} };

	if (data) {
		opts.headers = {
			"Content-Type": "application/json",
		};
		opts.body = JSON.stringify(data);
	}

	if (token) {
		opts.headers['Authorization'] = `Token ${token}`;
	}

	const response = await fetch(`${base}/${path}`, opts);

	if (response.ok || response.status === 422) {
		const result = await response.json();
		return result ? result : {};
	}

	throw error(response.status);
}

export function get(path: any, data?: any, token?: any) {
	return send({ method: 'GET', path, data, token });
}

export function del(path: any, token: any, data?: any, ) {
	return send({ method: 'DELETE', path, data, token });
}

export function post(path: any, data: any, token?: any) {
	return send({ method: 'POST', path, data, token });
}

export function put(path: any, data: any, token: any) {
	return send({ method: 'PUT', path, data, token });
}