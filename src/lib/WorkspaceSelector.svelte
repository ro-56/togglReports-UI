<script lang="ts">
  import Select, { Option } from '@smui/select';
  import type { OptionItem } from './types';
  import { beforeUpdate, onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri"
  import type { Workspace } from '$lib/types';
  

  export let available_options: Workspace[] = [];
  export let label: string = "";
  export let token: string = "";
  export let style: string = "";

  export let selected_value: number = null;
  export let initial_value: number = null;


  export async function update() {
    console.log("updating workspaces");
    available_options = await invoke("get_workspaces", {"apiToken" : token});
  }

  onMount(async () => {
    await update();
  });

</script>



<Select 
  variant="filled"
  label={label}
  bind:value={selected_value}
  bind:style
>

  {#if initial_value && !selected_value}
    <Option value={initial_value} disabled>{initial_value}</Option>
  {/if}

  {#each available_options as mode}
    <Option value={mode.id}>{mode.id+" - "+mode.name}</Option>
  {/each}

</Select>


