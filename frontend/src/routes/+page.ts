// export async function load({ fetch }) {
//     try {
//         const res = await fetch("https://swapi.dev/api/people/1", { method: 'GET' });
//         const item = await res.text();
//         console.log(item);
//         return { item }

//     } catch (error) {
//         console.log(error)
//     }

// }

export async function load({ fetch }) {
    try {
        const res = await fetch("http://127.0.0.1:6006", { method: 'GET' });
        const item = await res.text();
        console.log(item);
        return { item }

    } catch (error) {
        console.log(error)
    }

}

// export async function load({fetch}) {
//     const res = await fetch("http://localhost:6006",{method: 'GET'});
//     const item = await res.text();
//     console.log(item);
//     return {item}
// }