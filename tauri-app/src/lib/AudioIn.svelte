<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"

  let name = "";
  let greetMsg = ""

  async function get_input_audio_list(){
    greetMsg = await invoke("get_input_audio_list")
  }

  get_input_audio_list();
</script>
..
{#each greetMsg as msg}
    <div>{msg}</div>
{/each}