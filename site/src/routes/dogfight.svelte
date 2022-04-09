<section class="flex flex-col items-center bg-gradient-to-r bg-slate-900 min-h-screen">
    <div class="relative mx-auto box-content p-3 bg-slate-800 rounded-lg top-10" style="width:64vh;height:64vh;"></div>
    <form on:submit|preventDefault={spawn}>
    <input type="submit" style="display: none" />
    <input bind:value={name} type="text" placeholder="Docker image name" class="px-3 py-3 placeholder-slate-600 text-slate-200 relative bg-slate-800 rounded text-sm top-16 b-3" style="width:calc(64vh + 1.25rem)"/>
    </form>
</section>

<script>
    import { onMount } from 'svelte';
    let socket;
    let name;
    function spawn () {
      // post request here
    }
onMount(() => {

  socket = new WebSocket("ws://localhost:8000/ws")
  socket.addEventListener("open", ()=> {
    console.log("Opened"); socket.send("ping");
  })
  socket.addEventListener("message", (event)=> {
    console.log("Message", event)
  })
})
</script>