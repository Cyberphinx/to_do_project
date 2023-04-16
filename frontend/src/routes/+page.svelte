<script lang="ts">
	import { enhance } from '$app/forms';

	export let data;
	console.log(data);
	// const { item } = data;
</script>

<!-- <h1>{item}</h1> -->

<h1>This is the home page</h1>

<form use:enhance method="POST" action="?/logout">
	<button class="btn btn-outline-danger">Or click here to logout</button>
</form>

