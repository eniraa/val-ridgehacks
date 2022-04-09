<script>
	import { onMount } from 'svelte';
	let socket;
	let name;
	let players = [];

	function remToPixels(rem) {
		return rem * parseFloat(getComputedStyle(document.documentElement).fontSize);
	}

	function vhToPixels(vh) {
		return (vh * document.documentElement.clientHeight) / 100;
	}

	function spawn() {
		fetch('http://127.0.0.1:9000', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*' },
			body: JSON.stringify({ name: name })
		}).then((res) => {
			console.log('Request complete! response:', res);
		});

		name = '';
	}

	onMount(() => {
		socket = new WebSocket('ws://127.0.0.1:9001/ws');
		socket.addEventListener('open', () => {
			console.log('Opened');
		});
		socket.addEventListener('message', (event) => {
			console.log('Message', event);
			players = JSON.parse(event.data);
		});
	});
</script>

<section class="flex flex-col items-center bg-gradient-to-r bg-slate-900 min-h-screen">
	<div
		class="relative mx-auto box-content p-3 bg-slate-800 rounded-lg top-10"
		style="width:64vh;height:64vh;"
	>
		{#each players as player}
			{#if player.health > 0 && player.energy > 0}
				{#if Math.abs(player["kinematics"]["location"][0]) < 2000.0 && Math.abs(player["kinematics"]["location"][1]) < 2000.0}
				<div
					class="absolute bg-slate-600 rounded-full"
					style="height:5px;width:5px;left:calc({(player['kinematics']['location'][0] *
						(remToPixels(0.375) + vhToPixels(32.0))) /
						2000.0}px + 32vh + 0.375rem);top:calc({(player['kinematics']['location'][1] *
						(remToPixels(0.375) + vhToPixels(32.0))) /
						2000.0}px + 32vh + 0.375rem);"
				/>
                {/if}
			{/if}
		{/each}
	</div>
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
