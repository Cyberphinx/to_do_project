<script lang="ts">
	//OPTION 1 - client-side fetch without sveltekit:
	// import {onMount} from "svelte";

	// let sms = "";
	// onMount(async() => {
	//     sms = await getText();
	// })

	// const getText = async () => {
	//     const res = await fetch("http://localhost:6006");
	//     const data = await res.text();
	//     console.log(data);
	//     return data;
	// }

	//OPTION 2 - server-side fetch with sveltekit:
	export let data;
	console.log(data);
	const { item } = data;
</script>

<h1>Welcome to Frontend</h1>
<p>I'm from server-side fetch (with sveltekit): {item}</p>
