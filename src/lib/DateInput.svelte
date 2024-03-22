<script lang="ts">
    // import { DateInput } from 'date-picker-svelte';
    import Textfield from "@smui/textfield";
    import Icon from "@smui/textfield/icon";
    import { beforeUpdate } from 'svelte';
    
    
    export let date: string = "";
    export let initial_date: string = undefined;
    export let label: string = undefined;

    beforeUpdate(() => {
        label = label || undefined;
        date = date || initial_date || "";
    });

    function clicked(e) {
        if (e.target?.nodeName === "INPUT") {
            e.target.showPicker()
        }
    }

</script>


<div class="content">
  <Textfield 
    variant="outlined" 
    type="date"
    label={label}
    required
    on:click={clicked}
    bind:value={date}
  >
    <Icon class="material-icons" slot="trailingIcon">today</Icon>
  </Textfield>
</div>

<style>
  .content {
    padding: 5px 5px 10px;
    width: auto;
  }
</style>