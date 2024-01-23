<script lang="ts">
  import { getCWD } from '$lib/api/system.api';
  import { APISystemExists } from '$lib/store/api.store';
  import { darkMode } from '$lib/store/app.store';
  import '../app.css';
  import Sidebar from './Sidebar.svelte';
  
  $: if ($APISystemExists) {
    (async () => {
      try {
        console.log(await getCWD());
        
      } catch (error) {
        console.log(error);
        if (error instanceof Error) {
          console.error(error.message);
        }
      }
    })();
  }
</script>

<div id="app-root" class:dark={$darkMode}
  class="h-full flex flex-row"
>
  <Sidebar />
  <main id="main-content" class="w-full h-full">
    <slot />
  </main>
</div>

<style lang="postcss">
  #app-root.dark {
    @apply bg-nnrygray-700 text-white;
  }
</style>
