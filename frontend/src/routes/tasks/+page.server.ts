import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

// export function load({ locals }) {
// 	if (!locals.user) throw redirect(302, '/login');
// }