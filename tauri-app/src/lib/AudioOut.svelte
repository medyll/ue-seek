<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"

  let name = "";
  let greetMsg = ""

  async function get_output_audio_device(){
    greetMsg = await invoke("get_output_audio_device")
  }

  get_output_audio_device();
</script>
..
{#each greetMsg as msg}
    <div>{msg}</div>
{/each}