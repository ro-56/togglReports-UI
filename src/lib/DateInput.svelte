<script lang="ts">
    // import { DateInput } from 'date-picker-svelte';
    import Textfield from "@smui/textfield";
    import Icon from "@smui/textfield/icon";
    import { beforeUpdate } from 'svelte';
    
    
    export let date;
    export let label;

    beforeUpdate(() => {
        label = label || undefined;
    });

    function clicked(e) {
        if (e.target?.nodeName === "INPUT") {
            e.target.showPicker()
        }
    }

</script>


<div class="content">
  <Textfield variant="outlined" 
            type="date"
            label={label}
            on:click={clicked}
            bind:value={date}>
    <Icon class="material-icons" slot="trailingIcon">today</Icon>
  </Textfield>
</div>

<style>
  .content {
    margin: 10px 10px 14px;
    width: auto;
  }
</style>