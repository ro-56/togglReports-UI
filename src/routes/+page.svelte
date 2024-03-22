<script lang="ts">
    import Button from '@smui/button';
    import { invoke } from "@tauri-apps/api/tauri"
    import { getVersion } from '@tauri-apps/api/app';

    import ModeSelector from '$lib/ModeSelector.svelte'
    import DateInput from '$lib/DateInput.svelte'
    import Snackbar from "$lib/Snackbar.svelte";
    import type { OptionItem, LastUsedOptions } from '$lib/types';
    import { onMount } from 'svelte';

    let APP_VERSION: string;

    let selected_start_date: string | null = null;
    let selected_end_date: string | null = null;
    let selected_option: OptionItem;
    let requires_dates: boolean = false;
    let last_used_options: LastUsedOptions = {};

    let snackbar: Snackbar;
    let snackbar_type: string = "";
    let snackbar_text: string = "";

    let options: OptionItem[] = [];

    async function run(){
        if (requires_dates && (!selected_start_date || !selected_end_date)) {
            snackbar_type = "error";
            snackbar_text = "Error: Please select a start and end date.";
            snackbar.open();
            return;
        }

        let res = "";
        try {
            res = await invoke(
                "run", 
                {"command": { ...selected_option, start_date: selected_start_date, end_date: selected_end_date }});
        } catch (error) {
            console.error(error);
            snackbar_type = "error";
            snackbar_text = "Error: Please check your inputs.";
            snackbar.open();
            return;
        }
           
        snackbar_type = "success";
        snackbar_text = res;
        snackbar.open();

        last_used_options = await invoke("get_last_used_options", { })
    }
  
    async function init() {
        options = await invoke("get_available_commands", { })
        APP_VERSION = await getVersion();
    }
  
    onMount(() => {
        init();
    });
  
</script>
  

<Snackbar
    bind:this={snackbar}
    type={snackbar_type}
    text={snackbar_text}>
</Snackbar>

<div class="page">
    <h1>Toggl Reports</h1>

    <div>
        <div class="row">
        <a href="https://track.toggl.com/" target="_blank">
            <img src="toggl_track_pink.svg" class="logo toggl-pink" alt="Toggl Track Logo" />
        </a>
        </div>

        <p>
            Select a time period to generate a report.
        </p>
    </div>

    <div>
        <ModeSelector label="Mode" initial_command={last_used_options.command} bind:selected_option={selected_option} bind:requires_dates={requires_dates} available_options={options}/>
        
        {#if requires_dates}
        <div class="row">
            <DateInput initial_date={last_used_options?.start_date} label="Start date" bind:date={selected_start_date}/>
            <DateInput initial_date={last_used_options?.end_date} label="End date" bind:date={selected_end_date}/>
        </div>
        {/if}

        <div class="button-confirm">
        <Button on:click={run} variant="unelevated">
            Confirm
        </Button>
        </div>
    </div>

    <div class="version">
        {#if APP_VERSION}
            <p>Version: {APP_VERSION}</p>
        {/if}
    </div>
</div>

<style>
  
    .page {
      transition: 0.15s linear;
      margin: 0;
      display: flex;
      flex-direction: column;
      justify-content: center;
      text-align: center;
    }
    .row {
      display: flex;
      justify-content: center;
    }
  
    .button-confirm {
      padding: 5px 5px 10px;
    }
  
    .version {
      font-size: 0.75rem;
      color: #404040;
    }
  
    .logo {
      height: 6em;
      padding: 1.5em;
      will-change: filter;
      transition: 0.75s;
    }
    .logo.toggl-pink:hover {
      filter: drop-shadow(0 0 2em #E57CD8);
    }
  
    /* .logo.toggl-purple:hover {
      filter: drop-shadow(0 0 2em #2C1138);
    } */
</style>
