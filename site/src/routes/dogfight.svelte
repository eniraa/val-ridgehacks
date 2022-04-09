<script>
	import { onMount } from 'svelte';
	let socket;
	let name;
	function spawn() {
		fetch('http://127.0.0.1:9000', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*' },
			body: JSON.stringify({ name: name })
		}).then((res) => {
			console.log('Request complete! response:', res);
		});

        name = "";
	}

	onMount(() => {
		socket = new WebSocket('ws://127.0.0.1:9001/ws');
		socket.addEventListener('open', () => {
			console.log('Opened');
		});
		socket.addEventListener('message', (event) => {
			console.log('Message', event);
		});
	});
</script>

<section class="flex flex-col items-center bg-gradient-to-r bg-slate-900 min-h-screen">
	<div
		class="relative mx-auto box-content p-3 bg-slate-800 rounded-lg top-10"
		style="width:64vh;height:64vh;"
	/>
	<form on:submit|preventDefault={spawn}>
		<input type="submit" style="display: none" />
		<input
			bind:value={name}
			type="text"
			placeholder="Docker image name"
			class="px-3 py-3 placeholder-slate-600 text-slate-200 relative bg-slate-800 rounded text-sm top-16 b-3"
			style="width:calc(64vh + 1.25rem)"
		/>
	</form>
</section>
