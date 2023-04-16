<script lang="ts">
	import { applyAction, enhance } from '$app/forms';

	export let form;
	$: console.log(form);

	export let data;
</script>

<svelte:head>
	<title>Sign up • To-do-project</title>
</svelte:head>

<h1>Login</h1>
<form
	method="POST"
	use:enhance={({ form }) => {
		// this runs before form submission to server
		return async ({ result, update }) => {
			// this runs after form submission to server
			if (result.type === 'success') {
				form.reset();
			}
			if (result.type === 'failure') {
				await applyAction(result);
			}
			update();
		};
	}}
>
	<fieldset class="form-group">
		<input
			name="username"
			type="text"
			required
			placeholder="Your Name"
		/>
	</fieldset>

	<fieldset class="form-group">
		<input name="password" type="password" required placeholder="Password" />
	</fieldset>
	<button type="submit">Sign up</button>
</form>

<p>Response: {data}</p>
