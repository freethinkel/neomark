<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { path, fs, window } from "@tauri-apps/api";
  import Preview from "./lib/Preview/View.svelte";
  import { watch } from "tauri-plugin-fs-watch-api";
  import PickFile from "./lib/PickFile.svelte";

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
  const pickFile = async (path?: string) => {
    if (unwatch) {
      unwatch();
    }

    unwatch = await watch(path, { recursive: true }, () => {
      pickFile(path);
    });
    const content = await fs.readTextFile(path);
    fileContent = content;
    hasFile = true;
    window.appWindow.setTitle(`neomark - ${path}`);
  };

  onMount(async () => {
    const path = await getStartedFile();
    if (path) {
      pickFile(path);
    }
  });
</script>

{#if hasFile}
  <Preview content={fileContent} />
{:else}
  <PickFile on:select={({ detail }) => pickFile(detail)} />
{/if}
