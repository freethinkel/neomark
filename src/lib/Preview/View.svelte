<script lang="ts">
  import { unified } from "unified";
  import remarkParse from "remark-parse";
  import remarkRehype from "remark-rehype";
  import rehypeStringify from "rehype-stringify";
  import rehypeHighlight from "rehype-highlight";
  import remarkGfm from "remark-gfm";
  import {join, dirname} from '@tauri-apps/api/path'

  import highlight from "highlight.js/lib/common";

  import "./styles.pcss";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

  export let content = "";
  export let path = ""

  let rendered = "";
  let wrapperEl: HTMLDivElement;

  $: {
    process(content).then((content) => {
      console.log(content);
      rendered = content;

      setTimeout(() => {
        const allIMages = wrapperEl.querySelectorAll("img")

        allIMages.forEach(async (img) => {
          const source = img.getAttribute("src");
          if (source.indexOf("http") === 0) {
            return
          }

          const newSource = await join(await dirname(path), source)
          img.src = convertFileSrc(newSource)
          })
        console.log(allIMages)
      })
    });
  }

  const process = async (content: string) => {
    const file = await unified()
      .use(remarkParse)
      .use(remarkGfm)
      .use(remarkRehype)
      .use(rehypeHighlight, {
        ignoreMissing: true,
        languages: highlight.listLanguages().reduce((acc, curr) => {
          acc[curr] = (_) => highlight.getLanguage(curr);
          return acc;
        }, {}),
      })
      .use(rehypeStringify)
      .process(content);


    return String(file);
  };
</script>

<div class="preview markdown-body" bind:this={wrapperEl}>
  {@html rendered}
</div>

<style lang="postcss">
  .preview {
    height: 100%;
    min-height: inherit;
    padding: 10px;
  }
</style>
