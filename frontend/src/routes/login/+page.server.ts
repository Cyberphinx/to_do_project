import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export async function load({ cookies }) {
	if (cookies.get('jwt')) throw redirect(307, '/');
}

export const actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();

		const body = await api.post('users/login', {
			user: {
				username: data.get('username'),
				password: data.get('password')
			}
		});

		if (body.errors) {
			return fail(401, body);
		}

		const value = btoa(JSON.stringify(body.user));
		cookies.set('jwt', value, { path: '/' });

		throw redirect(307, '/');
	}
};