<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { path, fs, window } from "@tauri-apps/api";
  import Preview from "./lib/Preview/View.svelte";
  import { watch } from "tauri-plugin-fs-watch-api";
  import PickFile from "./lib/PickFile.svelte";
    import Titlebar from "./lib/Titlebar.svelte";

  let fileContent = "";
  let hasFile = false;

  const getStartedFile = async () => {
    const { args } = await getMatches();

    if (!args.source.value) {
      return null;
    }

    const currentDir = String(await invoke("get_current_dir"));
    return await path.join(currentDir, String(args.source.value || ""));
  };

  let unwatch = null;
  let selectedPath: string | null = null
  let title = ""
  const pickFile = async (path?: string) => {
    selectedPath = path
    if (unwatch) {
      unwatch();
    }

    unwatch = await watch(path, { recursive: true }, () => {
      pickFile(path);
    });
    const content = await fs.readTextFile(path);
    fileContent = content;
    hasFile = true;
    title = `neomark - ${path}`;
    // window.appWindow.setTitle(`neomark - ${path}`);
  };

  onMount(async () => {
    const path = await getStartedFile();
    if (path) {
      pickFile(path);
    }
  });
</script>

<div class="wrapper">
  <Titlebar>{title}</Titlebar>
  <div class="body">
    {#if hasFile}
      <Preview content={fileContent} path={selectedPath} />
    {:else}
      <PickFile on:select={({ detail }) => pickFile(detail)} />
    {/if}
  </div>
</div>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  .body {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
</style>
