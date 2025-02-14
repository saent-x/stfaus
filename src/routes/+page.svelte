<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import MediaControl from "../lib/components/media_control.svelte";
  import LlmChatbox from "../lib/components/llm_chatbox.svelte";
  import Tabs from "../lib/components/tabs.svelte";
  import SavedPlaylist from "../lib/components/saved_playlist.svelte";
    import GeneratedPlaylist from "../lib/components/generated_playlist.svelte";

  let name = $state("");
  let greetMsg = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container h-screen relative flex flex-col overflow-y-scroll">
    <div class="flex flex-row flex-grow justify-center mt-4 pt-[env(safe-area-inset-bottom)] w-[100%] mx-auto pb-32">
        <Tabs tab_content_1={GeneratedPlaylist} tab_content_2={SavedPlaylist}/>
    </div>
    
    <div class="flex flex-col mb-4 p-4 items-center mt-auto w-[100%] bottom-0 left-0 right-0 fixed">
        <MediaControl/>
        <LlmChatbox/>
    </div>
    
</main>


<style>
    .container {
        height: 100vh;
        /* background-color: red; */
    }
</style>
