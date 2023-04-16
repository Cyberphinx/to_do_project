import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export async function load({ cookies }) {
	if (cookies.get('jwt')) throw redirect(307, '/');
}

// export async function load({ fetch }) {
//     try {
//         const user = {
//         			username: "archon",
//         			password: "12345"
//         		};
//         const res = await fetch("http://127.0.0.1:6006/api/v1/users", { 
//             method: 'POST',
//             headers: {
//                 "Content-Type": "application/json",
//             },
//             body: JSON.stringify(user),
//     });
//         const item = await res.statusText;
//         console.log(item);
//         return { item }

//     } catch (error) {
//         console.log(error)
//     }
// }

export const actions = {
    default: async ({ cookies, request, fetch }) => {
        try {
            const formData = await request.formData();
            const usernameData = formData.get("username");
            const passwordData = formData.get("password");

            // validation logic
            if (usernameData) {
                if (usernameData.toString().length < 3) {
                    return fail(400, {
                        error: true,
                        message: 'Name must be at least three characters',
                        usernameData
                    })
                }
            }

            const user = {
                username: usernameData,
                password: passwordData
            };

            // do something with the data

            // const response = await fetch("http://127.0.0.1:6006/api/v1/users", {
            //     method: 'POST',
            //     headers: {
            //         "Content-Type": "application/json",
            //     },
            //     body: JSON.stringify(user),
            // });

            // const status = response.statusText;
            // console.log(status);

            // const result = await response.json();
            // console.log(result);
            // return { result }

            const body = await api.post('api/v1/users', user);

            if (body.errors) {
                return fail(401, body);
            }

            const value = btoa(JSON.stringify(body.user));
            cookies.set('jwt', value, { path: '/' });

            throw redirect(307, '/');

        } catch (error) {
            console.log(error)
        }
    }
};