<script lang="ts">
  import Select, { Option } from '@smui/select';
  import type { OptionItem } from './types';
  import { beforeUpdate } from 'svelte';
  

  export let available_options: OptionItem[] = [];
  export let initial_command: string = "";
  export let label: string = "";
  export let requires_dates: boolean = false;
  export let selected_option: OptionItem = undefined;

  let selected_value: string = "";

  $: {
    requires_dates = available_options.find((option) => option.command === selected_value)?.requires_date_range;
    selected_option = available_options.find((option) => option.command === selected_value);
  }

  // beforeUpdate(() => {
  //   selected_value = selected_value
  //       || available_options.find((option) => option.command === initial_command)?.command
  //       || available_options[0]?.command
  //       || "";
  // });
</script>


<div class="content">
  <Select variant="outlined"
          label={label}
          bind:value={selected_value}
          required
          >
    {#each available_options as mode}
      <Option value={mode.command}>{mode.label}</Option>
    {/each}
  </Select>
</div>

<style>
  .content {
    padding: 5px 5px 10px;
    width: auto;
  }
</style>