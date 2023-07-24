<script lang="ts">
  import Select, { Option } from '@smui/select';
  
  type OptionItem = {
    idx: number;
    label: string;
    command: string;
    requires_date: boolean;
  }

  export let available_options: OptionItem[] = [];
  export let label: string = "";
  export let selected_option: OptionItem = undefined;

  let selected_value: string = "";

  $: selected_option = available_options.find((option) => option.command === selected_value);

</script>


<div class="content">
  <Select variant="outlined"
          label={label}
          bind:value={selected_value}
          >
    {#each available_options as mode}
      <Option value={mode.command}>{mode.label}</Option>
    {/each}
  </Select>
</div>

<style>
  .content {
    margin: 10px 0 14px;
    width: auto;
  }
</style>