<script  lang="ts">
    import Textfield from "@smui/textfield";
    import Button from '@smui/button';
    import { invoke } from "@tauri-apps/api/tauri"
    import type { Config } from "$lib/types";
    import Snackbar from "$lib/Snackbar.svelte";
    import Workspaces from "$lib/WorkspaceSelector.svelte";
    import { onMount } from "svelte";


    let api_token: string = "";
    let output_file_dir: string = "";
    let workspace_id: number = null;
    let sgu_name: string = "";
    let ignore_tag: string = "";
    let default_tag: string = "";

    let workspace_selector: Workspaces;

    let snackbar: Snackbar;
    let snackbar_type: string = "success";
    let snackbar_text: string = "Saved!";

    let has_invalid_api_token: boolean = false;
    let has_invalid_output_file_dir: boolean = false;
    let has_invalid_workspace_id: boolean = false;
    let has_invalid_sgu_name: boolean = false;
    let has_invalid_fields: boolean;
    $: has_invalid_fields = has_invalid_api_token || has_invalid_output_file_dir || has_invalid_workspace_id || has_invalid_sgu_name;
    $: has_invalid_api_token = api_token.length == 0;
    $: has_invalid_workspace_id = workspace_id == null;
    $: has_invalid_sgu_name = sgu_name.length == 0;

    async function submitConfig(){
        console.table({
            "api_token": has_invalid_api_token,
            "output_file_dir": has_invalid_output_file_dir,
            "workspace_id": has_invalid_workspace_id,
            "sgu_name": has_invalid_sgu_name
        });

        if (!has_invalid_fields) {
            try {
                let res = await invoke("set_config", { 
                    "cfg": {
                        "api_token": api_token,
                        "workspace_filter": workspace_id,
                        "output_file_dir": output_file_dir,
                        "sgu_name": sgu_name,
                        "ignore_tag": ignore_tag,
                        "default_tag": default_tag
                    }
                });

                if (res) {
                    snackbar_type = "success";
                    snackbar_text = "Saved!";
                    snackbar.open();
                    return;
                }
            } catch (error) {
                console.log(error);
            }
        }
        
        snackbar_type = "error";
        snackbar_text = "Error. Please check your inputs.";
        snackbar.open();

    }

    async function check_if_dir_is_valid() {
        if (output_file_dir.length == 0) {
            has_invalid_output_file_dir = true;
            return;
        }

        try {
            let res = await invoke(
                "check_if_dir_exists", 
                { "dir": output_file_dir }
            );
            if (res) {
                has_invalid_output_file_dir = false;
                return;
            }
        } catch (error) {
            console.log(error);
        }
        has_invalid_output_file_dir = true;
    }

    async function restore_defaults() {
        let _  = await invoke("restore_defaults", { });
        init();
    }

    onMount(async () => {
        await init();
        await workspace_selector.update();
    });

    async function init() {
        let curr_config: Config = await invoke("get_config", { })

        api_token = curr_config.api_token;
        output_file_dir = curr_config.output_file_dir;
        workspace_id = curr_config.workspace_filter;
        sgu_name = curr_config.sgu_name;
        ignore_tag = curr_config.ignore_tag;
        default_tag = curr_config.default_tag;

    }


</script>


<div class="page">

    <h1>Settings</h1>
    

    <div class="row">
        <Textfield
            label="API Token"
            type="password"
            variant="filled"
            required
            bind:value={api_token}
            bind:invalid={has_invalid_api_token}
            on:change={workspace_selector.update}
            style="min-width: 250px; width: 100%;"/>
    </div>
    
    <div class="row">
        <Workspaces 
            bind:this={workspace_selector}
            label="Workspace ID"
            bind:invalid={has_invalid_workspace_id}
            bind:token={api_token}
            bind:selected_value={workspace_id}
            bind:initial_value={workspace_id}
            style="min-width: 250px; width: 100%;"
        />
    </div>

    <div class="row hide-file-ui">
        <Textfield 
            label="Save output to"
            variant="filled"
            required
            bind:invalid={has_invalid_output_file_dir}
            on:change={check_if_dir_is_valid}
            bind:value={output_file_dir}
            style="min-width: 250px; width: 100%;"
            >
        </Textfield>
    </div>
    <!-- <div class="row">
        <Textfield 
            label="Workspace ID"
            type="number"
            required
            variant="filled"
            bind:value={workspace_id}
            bind:invalid={has_invalid_workspace_id}
            style="min-width: 250px; width: 100%;"/>
    </div> -->
    <div class="row">
        <Textfield 
            label="SGU Name"
            type="text"
            variant="filled"
            required
            bind:value={sgu_name}
            bind:invalid={has_invalid_sgu_name}
            style="min-width: 250px; width: 100%;"/>
    </div>
    <div class="row">
        <Textfield 
            label="Tag to ignore"
            type="text"
            variant="filled"
            bind:value={ignore_tag}
            style="min-width: 250px; width: 100%;"/>
    </div>
    <div class="row">
        <Textfield 
            label="Default tag"
            type="text"
            variant="filled"
            bind:value={default_tag}
            style="min-width: 250px; width: 100%;"/>
    </div>
    <div class="row">
        <Button 
            on:click={restore_defaults}
            variant="text"
            style="min-width: 250px; width: 100%;"
        >
            Restore Defaults
        </Button>
    </div>
    <div class="row">
        <Button 
            on:click={submitConfig}
            variant="unelevated"
            style="min-width: 250px; width: 100%;"
        >
            Save
        </Button>
    </div>
</div>

<Snackbar
    bind:this={snackbar}
    type={snackbar_type}
    text={snackbar_text}>
</Snackbar>


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
      padding: 10px;
      margin-left: 15%;
      margin-right: 15%;
    }
    .hide-file-ui :global(input[type='file']::file-selector-button) {
        display: none;
    }
    .hide-file-ui :global(:not(.mdc-text-field--label-floating) input[type='file']) {
        color: transparent;
    }

</style>
