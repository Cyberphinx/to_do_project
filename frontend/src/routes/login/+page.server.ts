export async function load({ fetch }) {
    try {
        const res = await fetch("http://127.0.0.1:6006/api/v1/users/login", { method: 'POST' });
        const item = await res.text();
        console.log(item);
        return { item }

    } catch (error) {
        console.log(error)
    }
}