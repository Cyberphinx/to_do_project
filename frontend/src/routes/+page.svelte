<script lang="ts">
    import {onMount} from "svelte";

    let sms = "";
    onMount(async() => {
        sms = await getText();
    })

    const getText = async () => {
        const res = await fetch("http://localhost:6006");
        const data = await res.text();
        console.log(JSON.stringify(data));
        return data;
    }

</script>

<h1>Welcome to Frontend</h1>
<p>{sms}</p>
