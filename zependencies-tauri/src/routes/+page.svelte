<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
</script>

{#await invoke('initialize')}
{:then}
  <p>Initialized!</p>
{:catch error}
  <p>Error: {JSON.stringify(error)}</p>
{/await}

<style>
</style>
