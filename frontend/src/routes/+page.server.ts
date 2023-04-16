import { error, fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

// export async function load({ fetch }) {
//     try {
//         const res = await fetch("http://127.0.0.1:6006", { method: 'GET' });
//         const item = await res.text();
//         console.log(item);
//         return { item }

//     } catch (error) {
//         console.log(error)
//     }
// }

export const actions = {
	logout: async ({ cookies, locals }) => {
		cookies.delete('jwt', { path: '/' });
	}
};